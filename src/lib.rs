#![deny(clippy::all)]

use anyhow::Result;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;

pub mod docx;
pub mod pdf;
pub mod spreadsheet;
pub mod thumbnail;

use thumbnail::ThumbnailOptions;

#[napi]
pub fn generate_thumbnail(path: String, width: u32, height: u32) -> Result<Buffer> {
  let opts = ThumbnailOptions { width, height };
  let bytes = thumbnail::generate_thumbnail(Path::new(&path), opts)?;
  Ok(bytes.into())
}
