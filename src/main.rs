use image::{ImageBuffer, Rgb, Rgba};

fn main() {
    let basis = image::open("/home/penguino/Pictures/jumpking/redcrown05.png")
        .expect("Could not find test-image")
        .into_rgba8();

    let basis_avg = rgb_avg(&basis);

    dbg!(basis_avg);

    let perp = image::open("/home/penguino/Pictures/screenshots/05-13-2025T07-21-48.png")
        .expect("Could not find test-image")
        .into_rgba8();

    let perp_avg = rgb_avg(&perp);

    dbg!(perp_avg);

    let cosine_similarity: f32 = {
        // Cosine similarity of two vectors is the dot product of the vectors
        // divided by the product of their lengths.

        let mut dot_product: u16 = 0;

        for i in 0..3 {
            dot_product += u16::from(perp_avg[i]) * (u16::from(basis_avg[i]));
        }

        let length_product = vector_length(basis_avg.0) * vector_length(perp_avg.0);

        f32::from(dot_product) / length_product
    };

    dbg!(cosine_similarity);
}

fn rgb_avg(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Rgb<u8> {
    let mut mode: Rgb<u8> = Rgb([0, 0, 0]);

    let min = 60;
    for Rgba { 0: rgb } in img
        .pixels()
        .filter(|Rgba([r, g, b, _])| *r >= min || *g >= min || *b >= min)
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
