use crate::document;
use crate::image as image_file;
use crate::presentation;
use crate::spreadsheet;
use crate::text;
use image::{DynamicImage, ImageBuffer, ImageError, Rgba, imageops};
use std::io::Write;
use std::path::Path;
use tempfile::Builder;

pub struct ThumbnailOptions {
  pub width: u32,
  pub height: u32,
}

enum InputType {
  Image,
  Document,
  Presentation,
  Spreadsheet,
  Text,
  Unsupported(String),
}

pub fn load_image(path: &Path) -> Result<DynamicImage, ImageError> {
  image_file::load(path)
}

fn detect_input_type(path: &Path) -> InputType {
  if let Ok(Some(kind)) = infer::get_from_path(path) {
    let mime = kind.mime_type();

    if mime.starts_with("image/") {
      return InputType::Image;
    }

    if mime == "application/pdf" {
      return InputType::Document;
    }

    if mime == "application/vnd.openxmlformats-officedocument.wordprocessingml.document" {
      return InputType::Document;
    }

    if mime == "application/vnd.openxmlformats-officedocument.presentationml.presentation"
      || mime == "application/vnd.ms-powerpoint"
    {
      return InputType::Presentation;
    }

    if mime == "text/csv"
      || mime == "application/vnd.ms-excel"
      || mime == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
      || mime == "application/vnd.ms-excel.sheet.macroenabled.12"
      || mime == "application/vnd.ms-excel.sheet.binary.macroenabled.12"
      || mime == "application/vnd.oasis.opendocument.spreadsheet"
    {
      return InputType::Spreadsheet;
    }

    if mime.starts_with("text/") {
      let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());

      if mime == "text/csv"
        || mime == "text/tab-separated-values"
        || extension.as_deref() == Some("csv")
        || extension.as_deref() == Some("tsv")
      {
        return InputType::Spreadsheet;
      }

      return InputType::Text;
    }

    return InputType::Unsupported(mime.to_string());
  }

  if let Some(ext) = path.extension().and_then(|extension| extension.to_str()) {
    match ext.to_ascii_lowercase().as_str() {
      "pdf" | "docx" | "doc" => return InputType::Document,
      "pptx" | "ppt" => return InputType::Presentation,
      "csv" | "tsv" | "xlsx" | "xls" | "xlsm" | "xlsb" | "ods" => {
        return InputType::Spreadsheet;
      }
      "txt" | "text" | "md" | "markdown" | "log" => return InputType::Text,
      "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" => {
        return InputType::Image;
      }
      _ => return InputType::Unsupported(ext.to_string()),
    }
  }

  InputType::Unsupported("unknown".to_string())
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
    let encoder = ::image::codecs::webp::WebPEncoder::new_lossless(&mut bytes);
    let rgba = img.to_rgba8();
    encoder.encode(rgba.as_raw(), img.width(), img.height(), img.color().into())?;
  }
  Ok(bytes)
}

pub fn generate_thumbnail(path: &Path, opts: ThumbnailOptions) -> anyhow::Result<Vec<u8>> {
  let img = match detect_input_type(path) {
    InputType::Image => load_image(path)?,
    InputType::Document => document::render_preview(path)?,
    InputType::Presentation => presentation::render_preview(path)?,
    InputType::Spreadsheet => spreadsheet::render_preview(path)?,
    InputType::Text => text::render_preview(path)?,
    InputType::Unsupported(kind) => {
      anyhow::bail!("Unsupported file format: {}", kind);
    }
  };

  let resized = resize_image(img, opts);
  let encoded = encode_webp(resized)?;
  Ok(encoded)
}

fn extension_for_mime_type(mime_type: &str) -> &'static str {
  match mime_type {
    "image/jpeg" => "jpg",
    "image/png" => "png",
    "image/gif" => "gif",
    "image/webp" => "webp",
    "image/bmp" => "bmp",
    "image/tiff" => "tiff",
    "application/pdf" => "pdf",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => "docx",
    "application/msword" => "doc",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation" => "pptx",
    "application/vnd.ms-powerpoint" => "ppt",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => "xlsx",
    "application/vnd.ms-excel" => "xls",
    "application/vnd.ms-excel.sheet.macroenabled.12" => "xlsm",
    "application/vnd.ms-excel.sheet.binary.macroenabled.12" => "xlsb",
    "application/vnd.oasis.opendocument.spreadsheet" => "ods",
    "text/csv" => "csv",
    "text/tab-separated-values" => "tsv",
    "text/markdown" => "md",
    mime if mime.starts_with("text/") => "txt",
    _ => "bin",
  }
}

pub fn generate_thumbnail_from_buffer(
  input: &[u8],
  mime_type: &str,
  opts: ThumbnailOptions,
) -> anyhow::Result<Vec<u8>> {
  let extension = extension_for_mime_type(mime_type);

  let mut temp = Builder::new()
    .prefix("genthumb-input-")
    .suffix(&format!(".{}", extension))
    .tempfile()
    .map_err(|error| anyhow::anyhow!("Failed to create temporary file: {}", error))?;

  temp
    .write_all(input)
    .map_err(|error| anyhow::anyhow!("Failed to write temporary file: {}", error))?;

  generate_thumbnail(temp.path(), opts)
}
