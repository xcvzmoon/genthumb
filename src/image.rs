use image::{DynamicImage, ImageError};
use std::path::Path;

pub fn load(path: &Path) -> Result<DynamicImage, ImageError> {
  image::open(path)
}
