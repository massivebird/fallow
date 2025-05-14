use image::DynamicImage;
use img_hash::{Hasher, HasherConfig, ImageHash, image};
use std::borrow::Cow;
use std::cell::OnceCell;
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
    let perp_path = Path::new("/home/penguino/Pictures/screenshots/05-13-2025T09-16-55.png");

    assert!(perp_path.is_file());

    let perp_img = image::open(perp_path)
        .expect("Could not find test-image")
        .into_rgb8();

    let screens_dir = Path::new("/home/penguino/Pictures/jumpking/");

    assert!(screens_dir.is_dir());

    println!("Loading screen images.");

    let screens = screens_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| Screen::new(&d.path()))
        .collect::<Vec<Screen>>();

    assert!(!screens.is_empty());

    println!("Initializing screen hashes. This takes a second.");

    let hasher = HasherConfig::new().to_hasher();

    // Initialize hashes.
    for screen in &screens {
        screen.init_hash(&hasher);
    }

    println!("All screen hashes completed.");

    let perp_hash = hasher.hash_image(&perp_img);

    let mut best_screen: Option<&Screen> = None;
    let mut best_dist: Option<u32> = None;

    for screen in screens.iter().skip(1) {
        let dist = perp_hash.dist(screen.hash.get().unwrap());

        if best_dist.is_none_or(|best| best > dist) {
            best_dist = Some(dist);
            best_screen = Some(screen);
        }
    }

    println!("Best match: {}", best_screen.unwrap().basename());
}
