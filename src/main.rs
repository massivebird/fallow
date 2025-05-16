use clap::{Arg, ValueHint};
use find_subimage::SubImageFinderState;
use img_hash::{Hasher, HasherConfig, ImageHash, image};
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

    // Now we do the video stuff.

    video::analyze(input_path, &screens, &hasher);
}
