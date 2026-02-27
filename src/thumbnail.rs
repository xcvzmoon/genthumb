use image::{DynamicImage, ImageBuffer, ImageError, Rgba, imageops};
use std::path::Path;

pub struct ThumbnailOptions {
  pub width: u32,
  pub height: u32,
}

pub fn detect_image_type(path: &Path) -> Option<&'static str> {
  match infer::get_from_path(path) {
    Ok(Some(kind)) => Some(kind.extension()),
    _ => None,
  }
}

pub fn load_image(path: &Path) -> Result<DynamicImage, ImageError> {
  image::open(path)
}

pub fn resize_image(img: DynamicImage, opts: ThumbnailOptions) -> DynamicImage {
  let resized: ImageBuffer<Rgba<u8>, Vec<u8>> = imageops::resize(
    &img,
    opts.width,
    opts.height,
    imageops::FilterType::Lanczos3,
  );
  DynamicImage::ImageRgba8(resized)
}

pub fn encode_webp(img: DynamicImage) -> Result<Vec<u8>, ImageError> {
  let mut bytes = Vec::new();
  {
    let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut bytes);
    let rgba = img.to_rgba8();
    encoder.encode(rgba.as_raw(), img.width(), img.height(), img.color().into())?;
  }
  Ok(bytes)
}

pub fn generate_thumbnail(path: &Path, opts: ThumbnailOptions) -> Result<Vec<u8>, ImageError> {
  let img = load_image(path)?;
  let resized = resize_image(img, opts);
  encode_webp(resized)
}
