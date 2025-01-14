pub mod color;
pub mod fonts;

use color::ColorResources;
use std::{path, env};

pub use fonts::load_fonts;

pub fn palette() -> ColorResources {
    ColorResources::default()
}

pub fn load_resources() -> path::PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    }
}