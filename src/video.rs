use image::{Rgb, RgbImage};
use std::io::Write;
use video_rs::decode::Decoder;

pub fn analyze<T: Write>(out_buf: &mut T, video_src: &std::path::Path, skip_every: usize) {
    video_rs::init().unwrap();

    let mut decoder = Decoder::new(video_src).expect("failed to create decoder");

    let mut img_buf = RgbImage::new(956, 720);

    for (frame_idx, frame) in decoder.decode_iter().enumerate() {
        if !(frame_idx == 0 || frame_idx % skip_every == 0) {
            continue;
        }

        if let Ok((_, frame)) = frame {
            let rgb: ndarray::ArrayView<_, _> = frame.slice(ndarray::s![.., .., ..]);

            // Construct this frame's RGB image.
            for (row_idx, rgb2) in rgb.slice(ndarray::s![.., .., ..]).outer_iter().enumerate() {
                for (col_idx, raw_rgb) in rgb2.slice(ndarray::s![.., ..]).outer_iter().enumerate() {
                    let rgb = Rgb([raw_rgb[0], raw_rgb[1], raw_rgb[2]]);

                    img_buf.put_pixel(col_idx as u32, row_idx as u32, rgb);
                }
            }

            writeln!(out_buf, "{frame_idx},{}", progress);
        } else {
            break;
        }
    }
}

fn progress(screen: &Screen, king_pos: (usize, usize)) -> f32 {
    let height = screen.area.height_offset() + king_pos.1 as u32;
    (height as f32) * 100.0 / ((SCREEN_HEIGHT * 45) as f32)
}

