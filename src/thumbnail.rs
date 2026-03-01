use crate::docx;
use crate::pdf;
use crate::pptx;
use crate::spreadsheet;
use crate::text;
use image::{DynamicImage, ImageBuffer, ImageError, Rgba, imageops};
use std::path::Path;

pub struct ThumbnailOptions {
  pub width: u32,
  pub height: u32,
}

enum InputType {
  Image,
  Pdf,
  Docx,
  Pptx,
  Spreadsheet,
  Text,
  Unsupported(String),
}

pub fn load_image(path: &Path) -> Result<DynamicImage, ImageError> {
  image::open(path)
}

fn detect_input_type(path: &Path) -> InputType {
  if let Ok(Some(kind)) = infer::get_from_path(path) {
    let mime = kind.mime_type();

    if mime.starts_with("image/") {
      return InputType::Image;
    }

    if mime == "application/pdf" {
      return InputType::Pdf;
    }

    if mime == "application/vnd.openxmlformats-officedocument.wordprocessingml.document" {
      return InputType::Docx;
    }

    if mime == "application/vnd.openxmlformats-officedocument.presentationml.presentation"
      || mime == "application/vnd.ms-powerpoint"
    {
      return InputType::Pptx;
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
      "pdf" => return InputType::Pdf,
      "docx" => return InputType::Docx,
      "pptx" | "ppt" => return InputType::Pptx,
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
    let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut bytes);
    let rgba = img.to_rgba8();
    encoder.encode(rgba.as_raw(), img.width(), img.height(), img.color().into())?;
  }
  Ok(bytes)
}

pub fn generate_thumbnail(path: &Path, opts: ThumbnailOptions) -> anyhow::Result<Vec<u8>> {
  let img = match detect_input_type(path) {
    InputType::Image => load_image(path)?,
    InputType::Pdf => pdf::render_first_page(path)?,
    InputType::Docx => docx::render_preview(path)?,
    InputType::Pptx => pptx::render_preview(path)?,
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
