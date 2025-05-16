use image::{ImageBuffer, Rgb, RgbImage};
use video_rs::decode::Decoder;

pub fn nth_frame(src: &std::path::Path, frame_idx: usize) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    video_rs::init().unwrap();

    let mut decoder = Decoder::new(src).expect("failed to create decoder");

    let mut img = RgbImage::new(956, 720);

    for frame in decoder.decode_iter().skip(frame_idx).take(1) {
        if let Ok((_, frame)) = frame {
            let rgb: ndarray::ArrayView<_, _> = frame.slice(ndarray::s![.., .., ..]);

            for (row_idx, rgb2) in rgb.slice(ndarray::s![.., .., ..]).outer_iter().enumerate() {
                for (col_idx, raw_rgb) in rgb2.slice(ndarray::s![.., ..]).outer_iter().enumerate() {
                    let rgb = Rgb([raw_rgb[0], raw_rgb[1], raw_rgb[2]]);

                    img.put_pixel(col_idx as u32, row_idx as u32, rgb);
                }
            }
        } else {
            break;
        }
    }

    img
}
