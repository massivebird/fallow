use find_subimage::SubImageFinderState;
use image::imageops::{self, resize};
use image::{DynamicImage, ImageBuffer, Rgb};
use img_hash::FilterType::Nearest;
use img_hash::{Hasher, HasherConfig, ImageHash, image};
use std::borrow::Cow;
use std::cell::OnceCell;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Screen {
    path: PathBuf,
    image: DynamicImage,
    hash: OnceCell<ImageHash>,
}

impl Screen {
    fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            image: image::open(path).expect("Could not find test-image"),
            hash: OnceCell::new(),
        }
    }

    fn init_hash(&self, hasher: &Hasher) {
        self.hash.set(hasher.hash_image(&self.image)).unwrap();
    }

    fn basename(&self) -> Cow<str> {
        self.path.file_stem().unwrap().to_string_lossy()
    }
}

fn main() {
    let perp_path = Path::new("/home/penguino/Pictures/screenshots/05-13-2025T09-31-37.png");

    assert!(perp_path.is_file());

    let input_img = image::open(perp_path)
        .expect("Could not find test-image")
        .into_rgb8();

    let screens_dir = Path::new("/home/penguino/Pictures/jumpking/screens/");

    assert!(screens_dir.is_dir());

    println!("Loading screen images.");

    let screens = screens_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| Screen::new(&d.path()))
        .collect::<Vec<Screen>>();

    assert!(!screens.is_empty());

    println!("Initializing screen hashes.");

    let hasher = HasherConfig::new().to_hasher();

    // Initialize hashes.
    for screen in &screens {
        screen.init_hash(&hasher);
    }

    let perp_hash = hasher.hash_image(&input_img);

    let mut best_screen: Option<&Screen> = None;
    let mut best_dist: Option<u32> = None;

    for screen in &screens {
        let dist = perp_hash.dist(screen.hash.get().unwrap());

        if best_dist.is_none_or(|best| best > dist) {
            best_dist = Some(dist);
            best_screen = Some(screen);
        }
    }

    println!("Best screen match: {}", best_screen.unwrap().basename());

    let mut finder = SubImageFinderState::new();

    let king_head_img = image::open("/home/penguino/Pictures/jumpking/king_head.png")
        .unwrap()
        .into_rgb8();

    let head_pos = find_patch(
        &mut finder,
        &input_img,
        &king_head_img,
        PatchType::FlipHorizontally,
    );

    dbg!(head_pos);

    let king_charge_img = image::open("/home/penguino/Pictures/jumpking/king_charge.png")
        .unwrap()
        .into_rgb8();

    let charge_pos = find_patch(&mut finder, &input_img, &king_charge_img, PatchType::NoFlip);

    dbg!(charge_pos);

    let dead_img = image::open("/home/penguino/Pictures/jumpking/king_dead.png")
        .unwrap()
        .into_rgb8();

    let dead_pos = find_patch(
        &mut finder,
        &input_img,
        &dead_img,
        PatchType::FlipHorizontally,
    );

    dbg!(dead_pos);
}

enum PatchType {
    NoFlip,
    FlipHorizontally,
}

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
        .find_subimage_positions(to_tuple(&src), to_tuple(&patch), 3)
        .to_vec();

    if matches!(patch_type, PatchType::NoFlip) {
        return locs.iter().max_by(|a, b| a.2.total_cmp(&b.2)).copied();
    }

    // Now search for the king looking in the opposite direction.
    imageops::flip_horizontal_in_place(&mut patch);

    locs.append(
        &mut finder
            .find_subimage_positions(to_tuple(&src), to_tuple(&patch), 3)
            .to_vec(),
    );

    locs.iter().max_by(|a, b| a.2.total_cmp(&b.2)).copied()
}
