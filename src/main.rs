use image::imageops::FilterType;
use image::{GenericImageView, ImageBuffer, Rgb};
use std::borrow::Cow;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Screen {
    path: PathBuf,
    rgb_avg: Rgb<u8>,
}

impl Screen {
    fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            rgb_avg: {
                let img = image::open(path)
                    .expect("Could not find test-image")
                    .into_rgb8();

                rgb_avg(&img)
            },
        }
    }

    fn basename(&self) -> Cow<str> {
        self.path.file_stem().unwrap().to_string_lossy()
    }
}

fn main() {
    let perp_path = Path::new("/home/penguino/Pictures/screenshots/05-13-2025T09-16-55.png");

    assert!(perp_path.is_file());

    let perp_img = image::open(perp_path)
        .expect("Could not find test-image")
        .into_rgb8();

    let perp_avg = rgb_avg(&perp_img);

    let screens_dir = Path::new("/home/penguino/Pictures/jumpking/");

    assert!(screens_dir.is_dir());

    let screens = screens_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| Screen::new(&d.path()))
        .collect::<Vec<Screen>>();

    assert!(!screens.is_empty());

    let calc_score = |s: &Screen| {
        let screen_img = my_crop_resize(
            &image::open(&s.path)
                .expect("Could not find test-image")
                .into_rgb8(),
        );

        let perp_img = my_crop_resize(&perp_img);

        let mut tmp = cosine_similarity(
            screen_img.pixels().next().unwrap().0,
            perp_img.pixels().next().unwrap().0,
        );

        for i in 1..50 {
            tmp += cosine_similarity(
                screen_img.pixels().nth(i).unwrap().0,
                perp_img.pixels().nth(i).unwrap().0,
            );
        }

        tmp
    };

    let mut best_screen = &screens[0];
    let mut best_score = calc_score(&screens[0]);

    for screen in screens.iter().skip(1) {
        dbg!(screen.basename());
        let score = calc_score(screen);
        dbg!(score);

        if score > best_score {
            best_score = score;
            best_screen = screen;
        }
    }

    println!("Best match: {}", best_screen.basename());
}

fn rgb_avg<T>(img: &T) -> Rgb<u8>
where
    T: GenericImageView<Pixel = Rgb<u8>>,
{
    let mut avg: Rgb<u8> = Rgb([0, 0, 0]);

    // Only compute average of a top-left square.
    // Calculated relative to image height.
    let bounds = img.height() / 5;

    for (_, _, Rgb { 0: rgb }) in img
        .pixels()
        .filter(|(x, y, _)| *x <= bounds && *y <= bounds)
    {
        for i in 0..3 {
            let sum: u16 = u16::from(avg[i]) + u16::from(rgb[i]);
            avg[i] = u8::try_from(sum.div_euclid(2)).unwrap();
        }
    }

    avg
}

fn vector_length(vec: [u8; 3]) -> f32 {
    let mut inner: u32 = 0;

    for val in vec {
        inner += u32::from(val).pow(2);
    }

    f32::sqrt(inner as f32)
}

fn cosine_similarity(a: [u8; 3], b: [u8; 3]) -> f32 {
    // Cosine similarity of two vectors is the dot product of the vectors
    // divided by the product of their lengths.

    let mut dot_product: u32 = 0;

    for i in 0..3 {
        dot_product += u32::from(a[i]) * (u32::from(b[i]));
    }

    let length_product = vector_length(b) * vector_length(a);

    if length_product == 0.0 {
        return 0.0;
    }

    dot_product as f32 / length_product
}

fn my_crop_resize(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = img.clone();

    let crop_size = img.height() / 5;

    image::imageops::crop(&mut img, 0, 0, crop_size, crop_size);

    let resize_size = 15;

    image::imageops::resize(&img, resize_size, resize_size, FilterType::Nearest)
}
