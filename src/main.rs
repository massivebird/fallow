use image::{GenericImageView, Rgb};
use std::path::Path;

struct Screen {
    basename: String,
    rgb_avg: Rgb<u8>,
}

impl Screen {
    fn new(path: &Path) -> Self {
        Self {
            basename: path.file_stem().unwrap().to_string_lossy().into_owned(),
            rgb_avg: {
                let img = image::open(path)
                    .expect("Could not find test-image")
                    .into_rgb8();

                rgb_avg(&img)
            },
        }
    }
}

fn main() {
    let perp_path = Path::new("/home/penguino/Pictures/screenshots/05-13-2025T09-23-37.png");

    assert!(perp_path.is_file());

    let perp = image::open(perp_path)
        .expect("Could not find test-image")
        .into_rgb8();

    let perp_avg = rgb_avg(&perp);

    let screens_dir = Path::new("/home/penguino/Pictures/jumpking/");

    assert!(screens_dir.is_dir());

    let screens = screens_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| Screen::new(&d.path()))
        .collect::<Vec<Screen>>();

    assert!(!screens.is_empty());

    let mut best_screen = &screens[0];
    let mut best_score = cosine_similarity(best_screen.rgb_avg.0, perp_avg.0);

    for screen in screens.iter().skip(1) {
        let cosine_similarity = cosine_similarity(screen.rgb_avg.0, perp_avg.0);

        if cosine_similarity > best_score {
            best_score = cosine_similarity;
            best_screen = screen;
        }
    }

    println!("Best match: {}", best_screen.basename);
}

fn rgb_avg<T>(img: &T) -> Rgb<u8>
where
    T: GenericImageView<Pixel = Rgb<u8>>,
{
    let mut avg: Rgb<u8> = Rgb([0, 0, 0]);

    // Only compute average of a top-left square.
    // Calculated relative to image height.
    let bounds = img.height() / 5;

    for (_, _, Rgb { 0: rgb }) in img.pixels().filter(|(x, y, Rgb { 0: rgb })| {
        *x <= bounds && *y <= bounds && rgb.iter().any(|&v| v >= 60)
    }) {
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

    let mut dot_product: u16 = 0;

    for i in 0..3 {
        dot_product += u16::from(a[i]) * (u16::from(b[i]));
    }

    let length_product = vector_length(b) * vector_length(a);

    f32::from(dot_product) / length_product
}
