use clap::{Arg, ValueHint};
use find_subimage::SubImageFinderState;
use image::{ImageBuffer, Rgb, imageops};
use img_hash::{Hasher, HasherConfig, ImageHash, image};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct Screen {
    path: PathBuf,
    hash: ImageHash,
}

impl Screen {
    fn new(path: &Path, hasher: &Hasher) -> Self {
        Self {
            path: path.to_path_buf(),
            hash: hasher.hash_image(&image::open(path).expect("Could not find test-image")),
        }
    }

    fn basename(&self) -> Cow<str> {
        self.path.file_stem().unwrap().to_string_lossy()
    }
}

fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("img")
                .value_hint(ValueHint::FilePath)
                .required(true),
        )
        .get_matches();

    let perp_path = Path::new(matches.get_one::<String>("img").unwrap());

    assert!(perp_path.is_file());

    let input_img = image::open(perp_path)
        .expect("Could not find test-image")
        .into_rgb8();

    let screens_dir = Path::new("/home/penguino/Pictures/jumpking/screens/");

    assert!(screens_dir.is_dir());

    println!("Loading screen images.");

    // Use this hasher to hash all screen images.
    let hasher = HasherConfig::new().to_hasher();

    let screens = screens_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| Screen::new(&d.path(), &hasher))
        .collect::<Vec<Screen>>();

    assert!(!screens.is_empty());

    let perp_hash = hasher.hash_image(&input_img);

    let mut best_screen: Option<&Screen> = None;
    let mut best_dist: Option<u32> = None;

    for screen in &screens {
        let dist = perp_hash.dist(&screen.hash);

        if best_dist.is_none_or(|best| best > dist) {
            best_dist = Some(dist);
            best_screen = Some(screen);
        }
    }

    println!("Best screen match: {}", best_screen.unwrap().basename());

    let mut finder = SubImageFinderState::new();

    let king_pos = locate_king(&mut finder, &input_img);

    dbg!(king_pos);
}

enum PatchType {
    NoFlip,
    FlipHorizontally,
}

#[allow(clippy::needless_pass_by_value)]
fn find_patch(
    finder: &mut SubImageFinderState,
    src: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    patch: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    patch_type: PatchType,
) -> Option<(usize, usize, f32)> {
    // let mut patch = resize(
    //     patch,
    //     (src.width() as f32 * 0.020214).round() as u32,
    //     (src.height() as f32 * 0.021277).round() as u32,
    //     Nearest,
    // );

    let mut patch = patch.clone();

    let to_tuple: fn(&ImageBuffer<_, _>) -> (&Vec<u8>, usize, usize) =
        |img| (img.as_raw(), img.width() as usize, img.height() as usize);

    let mut locs = finder
        .find_subimage_positions(to_tuple(src), to_tuple(&patch), 3)
        .to_vec();

    if matches!(patch_type, PatchType::NoFlip) {
        return locs.iter().max_by(|a, b| a.2.total_cmp(&b.2)).copied();
    }

    // Now search for the king looking in the opposite direction.
    imageops::flip_horizontal_in_place(&mut patch);

    locs.append(
        &mut finder
            .find_subimage_positions(to_tuple(src), to_tuple(&patch), 3)
            .to_vec(),
    );

    locs.iter().max_by(|a, b| a.2.total_cmp(&b.2)).copied()
}

#[allow(clippy::similar_names)]
fn locate_king(
    finder: &mut SubImageFinderState,
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> Option<(usize, usize)> {
    let king_head_img = image::open("/home/penguino/Pictures/jumpking/king_head.png")
        .unwrap()
        .into_rgb8();

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
