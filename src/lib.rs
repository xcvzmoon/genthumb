#![deny(clippy::all)]

use anyhow::Result;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;

pub mod document;
pub mod image;
pub mod presentation;
pub mod spreadsheet;
pub mod text;
pub mod thumbnail;

use thumbnail::ThumbnailOptions;

/// Generates a WebP thumbnail for a supported file path or in-memory bytes.
///
/// Supports image, document, presentation, spreadsheet, and text inputs.
///
/// @param input - Absolute or relative path to the source file, or source bytes as a Node.js `Buffer`.
/// @param width - Target thumbnail width in pixels.
/// @param height - Target thumbnail height in pixels.
/// @param mime_type - Optional MIME type when `input` is a `Buffer` (for example `image/jpeg` or `text/plain`).
/// @returns A WebP-encoded thumbnail as a Node.js `Buffer`.
///
/// @example
/// ```ts
/// import { writeFileSync } from 'node:fs';
/// import { generateThumbnail } from 'genthumb';
///
/// const output = generateThumbnail('./tests/documents/test-image.jpeg', 320, 240);
/// writeFileSync('./thumbnail.webp', output);
///
/// const source = Buffer.from('hello from memory');
/// const fromBuffer = generateThumbnail(source, 320, 240, 'text/plain');
/// writeFileSync('./thumbnail-from-buffer.webp', fromBuffer);
/// ```
#[napi]
pub fn generate_thumbnail(
  input: Either<String, Buffer>,
  width: u32,
  height: u32,
  mime_type: Option<String>,
) -> Result<Buffer> {
  let opts = ThumbnailOptions { width, height };
  let bytes = match input {
    Either::A(path) => thumbnail::generate_thumbnail(Path::new(&path), opts)?,
    Either::B(buffer) => {
      let resolved_mime_type = mime_type
        .or_else(|| infer::get(buffer.as_ref()).map(|kind| kind.mime_type().to_string()))
        .unwrap_or_else(|| "application/octet-stream".to_string());

      thumbnail::generate_thumbnail_from_buffer(buffer.as_ref(), &resolved_mime_type, opts)?
    }
  };

  Ok(bytes.into())
}
