use crate::{SCREEN_HEIGHT, Screen};
use find_subimage::SubImageFinderState;
use image::{ImageBuffer, Rgb, RgbImage, imageops::resize};
use img_hash::Hasher;
use video_rs::decode::Decoder;

enum PatchType {
    NoFlip,
    FlipHorizontally,
}

pub fn analyze(video_src: &std::path::Path, screens: &[Screen], hasher: &Hasher) {
    video_rs::init().unwrap();

    let mut finder = SubImageFinderState::new().with_backend(find_subimage::Backend::Scalar {
        threshold: 0.5,
        step_x: 1,
        step_y: 1,
    });

    let mut decoder = Decoder::new(video_src).expect("failed to create decoder");

    let skip_every = 500;

    let mut img_buf = RgbImage::new(956, 720);

    for (frame_idx, frame) in decoder.decode_iter().enumerate() {
        if !(frame_idx == 0 || frame_idx % skip_every == 0) {
            continue;
        }

        let Ok((_, frame)) = frame else {
            println!("Failed to load frame {frame_idx}.");
            break;
        };

        let rgb: ndarray::ArrayView<_, _> = frame.slice(ndarray::s![.., .., ..]);

        // Construct this frame's RGB image.
        for (row_idx, rgb2) in rgb.slice(ndarray::s![.., .., ..]).outer_iter().enumerate() {
            for (col_idx, raw_rgb) in rgb2.slice(ndarray::s![.., ..]).outer_iter().enumerate() {
                let rgb = Rgb([raw_rgb[0], raw_rgb[1], raw_rgb[2]]);

                img_buf.put_pixel(col_idx as u32, row_idx as u32, rgb);
            }
        }

        let Some(screen) = locate_screen(screens, &img_buf, hasher) else {
            continue;
        };

        let Some(king_pos) = locate_king(&mut finder, &img_buf) else {
            continue;
        };

        // writeln!(&mut out_buf, "{frame_idx},{}", progress(screen, king_pos));
        println!("{frame_idx},{}", progress(screen, king_pos));
    }
}

fn progress(screen: &Screen, king_pos: (usize, usize)) -> f32 {
    let height = screen.area.height_offset() + king_pos.1 as u32;
    (height as f32) * 100.0 / ((SCREEN_HEIGHT * 45) as f32)
}

fn locate_screen<'a>(
    screens: &'a [Screen],
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    hasher: &Hasher,
) -> Option<&'a Screen> {
    let img_hash = hasher.hash_image(img);

    let mut best_screen: Option<&Screen> = None;
    let mut best_dist: Option<u32> = None;

    for screen in screens {
        // dbg!(screen.to_string());
        let dist = img_hash.dist(&screen.hash);
        // dbg!(dist);

        if best_dist.is_none_or(|best| best > dist) {
            best_dist = Some(dist);
            best_screen = Some(screen);
        }
    }

    best_screen
}

#[allow(clippy::similar_names)]
fn locate_king(
    finder: &mut SubImageFinderState,
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> Option<(usize, usize)> {
    let king_head_img = image::open("/home/penguino/Pictures/jumpking/king_head.png")
        .unwrap()
        .into_rgb8();

    let king_head_img = resize(
        &king_head_img,
        (img.width() as f32 * 0.020214).round() as u32,
        (img.height() as f32 * 0.021277).round() as u32,
        img_hash::FilterType::Nearest,
    );

    let mut locs = vec![find_patch(
        finder,
        img,
        &king_head_img,
        PatchType::FlipHorizontally,
    )];

    let king_charge_img = image::open("/home/penguino/Pictures/jumpking/king_charge.png")
        .unwrap()
        .into_rgb8();

    locs.push(find_patch(finder, img, &king_charge_img, PatchType::NoFlip));

    let king_dead_img = image::open("/home/penguino/Pictures/jumpking/king_dead.png")
        .unwrap()
        .into_rgb8();

    locs.push(find_patch(
        finder,
        img,
        &king_dead_img,
        PatchType::FlipHorizontally,
    ));

    locs.iter()
        .filter(|o| o.is_some())
        .max_by(|a, b| a.unwrap().2.total_cmp(&b.unwrap().2))
        .copied()
        .map(|o| (o.unwrap().0, o.unwrap().1))
}

#[allow(clippy::needless_pass_by_value)]
fn find_patch(
    finder: &mut SubImageFinderState,
    src: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    patch: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    patch_type: PatchType,
) -> Option<(usize, usize, f32)> {
    let patch = patch.clone();

    let to_tuple: fn(&ImageBuffer<_, _>) -> (&Vec<u8>, usize, usize) =
        |img| (img.as_raw(), img.width() as usize, img.height() as usize);

    let mut locs = finder
        .find_subimage_positions(to_tuple(src), to_tuple(&patch), 3)
        .to_vec();

    if matches!(patch_type, PatchType::NoFlip) {
        return locs.iter().max_by(|a, b| a.2.total_cmp(&b.2)).copied();
    }

    locs.append(
        &mut finder
            .find_subimage_positions(to_tuple(src), to_tuple(&patch), 3)
            .to_vec(),
    );

    locs.iter().min_by(|a, b| a.2.total_cmp(&b.2)).copied()
}
