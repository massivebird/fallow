use crate::Screen;
use image::imageops::FilterType;
use image::{ImageBuffer, Rgb};

pub fn calc_score(s: &Screen, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> f32 {
    let screen_img = my_crop_resize(
        &image::open(&s.path)
            .expect("Could not find test-image")
            .into_rgb8(),
    );

    let img = my_crop_resize(img);

    let mut tmp = cosine_similarity(
        screen_img.pixels().next().unwrap().0,
        img.pixels().next().unwrap().0,
    );

    for i in 1..50 {
        tmp += cosine_similarity(
            screen_img.pixels().nth(i).unwrap().0,
            img.pixels().nth(i).unwrap().0,
        );
    }

    tmp
}

fn my_crop_resize(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = img.clone();

    let crop_size = img.height() / 5;

    image::imageops::crop(&mut img, 0, 0, crop_size, crop_size);

    let resize_size = 15;

    image::imageops::resize(&img, resize_size, resize_size, FilterType::Nearest)
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

fn vector_length(vec: [u8; 3]) -> f32 {
    let mut inner: u32 = 0;

    for val in vec {
        inner += u32::from(val).pow(2);
    }

    f32::sqrt(inner as f32)
}
