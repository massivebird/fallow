use image::{ImageBuffer, Rgb};

struct Screen {
    basename: String,
    rgb_avg: Rgb<u8>,
}

impl Screen {
    fn new(basename: &str) -> Self {
        Self {
            basename: basename.to_owned(),
            rgb_avg: {
                let img = image::open(format!("/home/penguino/Pictures/jumpking/{basename}"))
                    .expect("Could not find test-image")
                    .into_rgb8();

                rgb_avg(&img)
            },
        }
    }
}

fn main() {
    let screens = [
        Screen::new("drain1.png"),
        Screen::new("drain2.png"),
        Screen::new("redcrown5.png"),
        Screen::new("redcrown4.png"),
        Screen::new("redcrown3.png"),
    ];

    let perp = image::open("/home/penguino/Pictures/screenshots/05-13-2025T08-27-54.png")
        .expect("Could not find test-image")
        .into_rgb8();

    let perp_avg = rgb_avg(&perp);

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

fn rgb_avg(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Rgb<u8> {
    let mut mode: Rgb<u8> = Rgb([0, 0, 0]);

    let min = 60;
    for Rgb { 0: rgb } in img
        .pixels()
        .filter(|Rgb([r, g, b])| *r >= min || *g >= min || *b >= min)
    {
        for i in 0..3 {
            let sum: u16 = u16::from(mode[i]) + u16::from(rgb[i]);
            mode[i] = sum.div_euclid(2) as u8;
        }
    }

    mode
}

fn vector_length(v: [u8; 3]) -> f32 {
    let mut inner: u32 = 0;

    for i in 0..3 {
        inner += u32::from(v[i]).pow(2);
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
