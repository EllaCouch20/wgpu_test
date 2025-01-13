pub mod color;
pub mod text;

use color::ColorResources;

pub use text::load_fonts;
pub use text::CustomText;

pub fn palette() -> ColorResources {
    ColorResources::default()
}
