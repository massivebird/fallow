use clap::{Arg, ValueHint};
use find_subimage::SubImageFinderState;
use image::{ImageBuffer, Rgb, imageops::resize};
use img_hash::{FilterType::Nearest, Hasher, HasherConfig, ImageHash, image};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

mod video;

const SCREEN_HEIGHT: u32 = 611;

#[derive(Debug)]
struct Screen {
    area: Area,
    path: PathBuf,
    hash: ImageHash,
}

impl Screen {
    fn new(path: &Path, area: Area, hasher: &Hasher) -> Self {
        Self {
            path: path.to_path_buf(),
            area,
            hash: hasher.hash_image(&image::open(path).expect("Could not find test-image")),
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (screen {})", self.area.name(), self.area.num())
    }
}

#[derive(Copy, Clone, Debug)]
enum Area {
    Tower(u32),
    Blue(u32),
    Chapel(u32),
    Stormwall(u32),
    Frontier(u32),
    Bargain(u32),
    False(u32),
    Drain(u32),
    Redcrown(u32),
}

impl Area {
    const fn height_offset(self) -> u32 {
        match self {
            Self::Tower(n) => (n + 39) * SCREEN_HEIGHT,
            Self::Blue(n) => (n + 36) * SCREEN_HEIGHT,
            Self::Chapel(n) => (n + 32) * SCREEN_HEIGHT,
            Self::Stormwall(n) => (n + 25) * SCREEN_HEIGHT,
            Self::Frontier(n) => (n + 19) * SCREEN_HEIGHT,
            Self::Bargain(n) => (n + 14) * SCREEN_HEIGHT,
            Self::False(n) => (n + 10) * SCREEN_HEIGHT,
            Self::Drain(n) => (n + 5) * SCREEN_HEIGHT,
            Self::Redcrown(n) => n * SCREEN_HEIGHT,
        }
    }

    fn name(self) -> String {
        match self {
            Self::Tower(_) => "The Tower".to_owned(),
            Self::Blue(_) => "Blue Ruin".to_owned(),
            Self::Chapel(_) => "Chapel Perilous".to_owned(),
            Self::Stormwall(_) => "Stormwall Pass".to_owned(),
            Self::Frontier(_) => "Great Frontier".to_owned(),
            Self::Bargain(_) => "Bargainburg".to_owned(),
            Self::False(_) => "False King's Keep".to_owned(),
            Self::Drain(_) => "Colossal Drain".to_owned(),
            Self::Redcrown(_) => "Redcrown Woods".to_owned(),
        }
    }

    const fn num(self) -> u32 {
        match self {
            Self::Tower(i)
            | Self::Blue(i)
            | Self::Chapel(i)
            | Self::Stormwall(i)
            | Self::Frontier(i)
            | Self::Bargain(i)
            | Self::False(i)
            | Self::Drain(i)
            | Self::Redcrown(i) => i,
        }
    }
}

fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("video")
                .value_hint(ValueHint::FilePath)
                .required(true),
        )
        .get_matches();

    let input_path = Path::new(matches.get_one::<String>("video").unwrap());

    assert!(input_path.is_file());

    let input_img = video::nth_frame(input_path, 0);

    let screens_dir = Path::new("/home/penguino/Pictures/jumpking/screens/");

    assert!(screens_dir.is_dir());

    // Use this hasher to hash all screen images.
    let hasher = HasherConfig::new().to_hasher();

    let screen_paths = screens_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| d.path())
        .collect::<Vec<PathBuf>>();

    let mut screens = vec![];

    macro_rules! add_screens {
        ($vec: expr, $area: expr, $prefix: expr) => {{
            let mut counter = 1;

            while let Some(path) = screen_paths
                .iter()
                .find(|p| p.ends_with(format!("{}{counter}.png", $prefix)))
            {
                $vec.push(Screen::new(path, $area(counter), &hasher));
                counter += 1;
            }
        }};
    }

    add_screens!(&mut screens, Area::Tower, "tower");
    add_screens!(&mut screens, Area::Blue, "blue");
    add_screens!(&mut screens, Area::Chapel, "chapel");
    add_screens!(&mut screens, Area::Stormwall, "storm");
    add_screens!(&mut screens, Area::Frontier, "frontier");
    add_screens!(&mut screens, Area::Bargain, "bargain");
    add_screens!(&mut screens, Area::False, "false");
    add_screens!(&mut screens, Area::Drain, "drain");
    add_screens!(&mut screens, Area::Redcrown, "redcrown");

    assert!(!screens.is_empty());

    let perp_hash = hasher.hash_image(&input_img);

    let mut best_screen: Option<&Screen> = None;
    let mut best_dist: Option<u32> = None;

    for screen in &screens {
        // dbg!(screen.to_string());
        let dist = perp_hash.dist(&screen.hash);
        // dbg!(dist);

        if best_dist.is_none_or(|best| best > dist) {
            best_dist = Some(dist);
            best_screen = Some(screen);
        }
    }

    println!("Area: {}", best_screen.unwrap());

    let mut finder = SubImageFinderState::new().with_backend(find_subimage::Backend::Scalar {
        threshold: 0.5,
        step_x: 1,
        step_y: 1,
    });

    let king_pos = locate_king(&mut finder, &input_img);

    dbg!(king_pos);

    let height = best_screen.unwrap().area.height_offset() + king_pos.unwrap().1 as u32;

    println!(
        "Progress: {:0.02}%",
        (height as f32) * 100.0 / ((SCREEN_HEIGHT * 45) as f32)
    );
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
        Nearest,
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
