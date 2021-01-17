#[cfg(feature = "use_serde")]
extern crate serde;

const TABLE_SIZE: usize = 6;
const IMAGE_SIZE: usize = 16;

mod image;
pub use image::Image;

mod table;
pub use table::Table;
