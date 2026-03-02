use anyhow::{Context, Result, anyhow};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use pdfium_auto::bind_bundled;
use pdfium_render::prelude::*;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

const DOC_PREVIEW_WIDTH: u32 = 1100;
const DOC_PREVIEW_HEIGHT: u32 = 1500;
const DOC_PADDING: u32 = 36;
const DOC_LINE_HEIGHT: u32 = 16;
const DOC_MAX_LINES: usize = 80;

enum DocumentKind {
  Pdf,
  Docx,
}

pub fn render_preview(path: &Path) -> Result<DynamicImage> {
  match detect_document_kind(path)? {
    DocumentKind::Pdf => render_pdf_preview(path),
    DocumentKind::Docx => render_docx_preview(path),
  }
}

fn detect_document_kind(path: &Path) -> Result<DocumentKind> {
  if let Ok(Some(kind)) = infer::get_from_path(path) {
    let mime = kind.mime_type();

    if mime == "application/pdf" {
      return Ok(DocumentKind::Pdf);
    }

    if mime == "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
      || mime == "application/msword"
    {
      return Ok(DocumentKind::Docx);
    }
  }

  if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
    return match extension.to_ascii_lowercase().as_str() {
      "pdf" => Ok(DocumentKind::Pdf),
      "docx" | "doc" => Ok(DocumentKind::Docx),
      _ => Err(anyhow!("Unsupported document format: {}", extension)),
    };
  }

  Err(anyhow!("Unsupported document format"))
}

fn render_pdf_preview(path: &Path) -> Result<DynamicImage> {
  let pdfium = bind_bundled().context("Failed to bind bundled PDFium library")?;
  let document = pdfium
    .load_pdf_from_file(path, None)
    .with_context(|| format!("Failed to load PDF: {}", path.display()))?;

  let page = document.pages().get(0).context("PDF has no pages")?;

  let bitmap = page
    .render_with_config(&PdfRenderConfig::new())
    .context("Failed to render PDF page")?;

  Ok(bitmap.as_image())
}

fn render_docx_preview(path: &Path) -> Result<DynamicImage> {
  let text = extract_docx_text(path)?;
  Ok(render_docx_text_preview(&text))
}

fn extract_docx_text(path: &Path) -> Result<String> {
  let file =
    File::open(path).with_context(|| format!("Failed to open DOCX file: {}", path.display()))?;
  let mut archive = ZipArchive::new(file)
    .with_context(|| format!("Failed to read DOCX ZIP: {}", path.display()))?;
  let mut document_xml = String::new();

  archive
    .by_name("word/document.xml")
    .context("Missing word/document.xml in DOCX file")?
    .read_to_string(&mut document_xml)
    .context("Failed to read word/document.xml")?;

  let mut reader = Reader::from_str(&document_xml);
  reader.config_mut().trim_text(true);

  let mut text = String::new();

  loop {
    match reader.read_event() {
      Ok(Event::Text(event)) => {
        let piece = event.decode().context("Failed to decode DOCX text")?;
        if !piece.trim().is_empty() {
          if !text.is_empty() && !text.ends_with(['\n', ' ']) {
            text.push(' ');
          }
          text.push_str(piece.as_ref());
        }
      }
      Ok(Event::End(event)) => {
        if event.name().as_ref() == b"w:p" && !text.ends_with('\n') {
          text.push('\n');
        }
      }
      Ok(Event::Eof) => break,
      Err(error) => return Err(anyhow!("Failed to parse DOCX XML: {}", error)),
      _ => {}
    }
  }

  let normalized = text
    .replace(['\r', '\t'], " ")
    .lines()
    .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join("\n");

  if normalized.is_empty() {
    return Ok("(empty document)".to_string());
  }

  Ok(normalized)
}

fn render_docx_text_preview(text: &str) -> DynamicImage {
  let mut image = ImageBuffer::from_pixel(
    DOC_PREVIEW_WIDTH,
    DOC_PREVIEW_HEIGHT,
    Rgba([249, 250, 251, 255]),
  );

  let chars_per_line = ((DOC_PREVIEW_WIDTH - (DOC_PADDING * 2)) / 9) as usize;
  let wrapped = wrap_docx_text(text, chars_per_line, DOC_MAX_LINES);

  fill_rect(
    &mut image,
    DOC_PADDING,
    DOC_PADDING,
    DOC_PREVIEW_WIDTH - (DOC_PADDING * 2),
    DOC_PREVIEW_HEIGHT - (DOC_PADDING * 2),
    Rgba([255, 255, 255, 255]),
  );

  draw_border(
    &mut image,
    DOC_PADDING,
    DOC_PADDING,
    DOC_PREVIEW_WIDTH - (DOC_PADDING * 2),
    DOC_PREVIEW_HEIGHT - (DOC_PADDING * 2),
    Rgba([209, 213, 219, 255]),
  );

  let mut y = DOC_PADDING + 20;
  for line in wrapped {
    draw_text_line(
      &mut image,
      DOC_PADDING + 16,
      y,
      &line,
      Rgba([31, 41, 55, 255]),
    );
    y += DOC_LINE_HEIGHT;

    if y + DOC_LINE_HEIGHT >= DOC_PREVIEW_HEIGHT - DOC_PADDING {
      break;
    }
  }

  DynamicImage::ImageRgba8(image)
}

fn wrap_docx_text(text: &str, width: usize, max_lines: usize) -> Vec<String> {
  let mut lines = Vec::new();

  for paragraph in text.lines() {
    let mut line = String::new();

    for word in paragraph.split_whitespace() {
      if line.is_empty() {
        line.push_str(word);
      } else if line.len() + 1 + word.len() <= width {
        line.push(' ');
        line.push_str(word);
      } else {
        lines.push(line);
        line = word.to_string();
      }

      if lines.len() >= max_lines {
        return lines;
      }
    }

    if !line.is_empty() {
      lines.push(line);
    }

    if lines.len() >= max_lines {
      return lines;
    }

    if !paragraph.is_empty() {
      lines.push(String::new());
    }
  }

  if lines.is_empty() {
    lines.push("(empty document)".to_string());
  }

  lines.truncate(max_lines);
  lines
}

fn draw_text_line(image: &mut RgbaImage, x: u32, y: u32, text: &str, color: Rgba<u8>) {
  let mut cursor_x = x;

  for character in text.chars() {
    if let Some(glyph) = BASIC_FONTS.get(character) {
      for (row_index, row_bits) in glyph.iter().enumerate() {
        for col_index in 0..8 {
          if (row_bits >> col_index) & 1 == 1 {
            let px = cursor_x + col_index;
            let py = y + row_index as u32;

            if px < image.width() && py < image.height() {
              image.put_pixel(px, py, color);
            }
          }
        }
      }
    }

    cursor_x += 9;
    if cursor_x + 8 >= DOC_PREVIEW_WIDTH - DOC_PADDING {
      break;
    }
  }
}

fn fill_rect(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, color: Rgba<u8>) {
  let x_end = (x + width).min(image.width());
  let y_end = (y + height).min(image.height());

  for yy in y..y_end {
    for xx in x..x_end {
      image.put_pixel(xx, yy, color);
    }
  }
}

fn draw_border(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, color: Rgba<u8>) {
  let x_end = (x + width).min(image.width() - 1);
  let y_end = (y + height).min(image.height() - 1);

  for px in x..=x_end {
    image.put_pixel(px, y, color);
    image.put_pixel(px, y_end, color);
  }

  for py in y..=y_end {
    image.put_pixel(x, py, color);
    image.put_pixel(x_end, py, color);
  }
}
