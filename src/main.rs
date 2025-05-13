use std::borrow::Cow;
use std::path::{Path, PathBuf};

mod score;

#[derive(Debug)]
struct Screen {
    path: PathBuf,
}

impl Screen {
    fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
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

    let screens = screens_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| Screen::new(&d.path()))
        .collect::<Vec<Screen>>();

    assert!(!screens.is_empty());

    let mut best_screen = &screens[0];
    let mut best_score = score::calc_score(&screens[0], &perp_img);

    for screen in screens.iter().skip(1) {
        dbg!(screen.basename());
        let score = score::calc_score(screen, &perp_img);
        dbg!(score);

        if score > best_score {
            best_score = score;
            best_screen = screen;
        }
    }

    println!("Best match: {}", best_screen.basename());
}
