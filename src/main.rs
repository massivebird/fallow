use image::{ImageBuffer, Rgb, Rgba};

fn main() {
    let basis = image::open("/home/penguino/Pictures/jumpking/redcrown05.png")
        .expect("Could not find test-image")
        .into_rgba8();

    let basis_mode = calc_mode(&basis);

    dbg!(basis_mode);

    let perp = image::open("/home/penguino/Pictures/screenshots/05-13-2025T07-21-48.png")
        .expect("Could not find test-image")
        .into_rgba8();

    let perp_mode = calc_mode(&perp);

    dbg!(perp_mode);

    let cosine_similarity: f32 = {
        let mut dot_product: u16 = 0;

        for i in 0..3 {
            dot_product += u16::from(perp_mode[i]) * (u16::from(basis_mode[i]));
        }

        let basis_length = vector_length(basis_mode.0);
        let perp_length = vector_length(perp_mode.0);

        let length_product = basis_length * perp_length;

        f32::from(dot_product) / length_product
    };

    dbg!(cosine_similarity);
}

fn calc_mode(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Rgb<u8> {
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
