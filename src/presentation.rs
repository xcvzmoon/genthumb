use anyhow::{Context, Result, anyhow};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

const PREVIEW_WIDTH: u32 = 1366;
const PREVIEW_HEIGHT: u32 = 768;
const PADDING: u32 = 32;
const LINE_HEIGHT: u32 = 18;
const MAX_LINES: usize = 28;

pub fn render_preview(path: &Path) -> Result<DynamicImage> {
  let lines = extract_slide_lines(path)?;
  Ok(render_slide_preview(&lines))
}

fn extract_slide_lines(path: &Path) -> Result<Vec<String>> {
  let file =
    File::open(path).with_context(|| format!("Failed to open PPTX file: {}", path.display()))?;
  let mut archive = ZipArchive::new(file)
    .with_context(|| format!("Failed to read PPTX ZIP: {}", path.display()))?;

  let mut slide_names = archive
    .file_names()
    .filter(|name| name.starts_with("ppt/slides/slide") && name.ends_with(".xml"))
    .map(ToOwned::to_owned)
    .collect::<Vec<_>>();

  slide_names.sort_by_key(|name| slide_index(name));

  let first_slide = slide_names
    .first()
    .ok_or_else(|| anyhow!("PPTX has no slide XML files"))?
    .clone();

  let mut slide_xml = String::new();
  archive
    .by_name(&first_slide)
    .with_context(|| format!("Missing slide data: {}", first_slide))?
    .read_to_string(&mut slide_xml)
    .with_context(|| format!("Failed to read {}", first_slide))?;

  parse_slide_text(&slide_xml)
}

fn slide_index(name: &str) -> u32 {
  let filename = name.rsplit('/').next().unwrap_or_default();
  let digits = filename
    .chars()
    .skip_while(|ch| !ch.is_ascii_digit())
    .take_while(|ch| ch.is_ascii_digit())
    .collect::<String>();

  digits.parse::<u32>().unwrap_or(u32::MAX)
}

fn parse_slide_text(slide_xml: &str) -> Result<Vec<String>> {
  let mut reader = Reader::from_str(slide_xml);
  reader.config_mut().trim_text(true);

  let mut lines = Vec::new();
  let mut current = String::new();

  loop {
    match reader.read_event() {
      Ok(Event::Text(event)) => {
        let piece = event.decode().context("Failed to decode PPTX text")?;
        let normalized = piece.split_whitespace().collect::<Vec<_>>().join(" ");

        if !normalized.is_empty() {
          if !current.is_empty() {
            current.push(' ');
          }
          current.push_str(&normalized);
        }
      }
      Ok(Event::End(event)) => {
        if event.name().as_ref() == b"a:p" && !current.is_empty() {
          lines.push(current.clone());
          current.clear();
        }
      }
      Ok(Event::Eof) => break,
      Err(error) => return Err(anyhow!("Failed to parse PPTX XML: {}", error)),
      _ => {}
    }
  }

  if !current.is_empty() {
    lines.push(current);
  }

  if lines.is_empty() {
    lines.push("(empty presentation)".to_string());
  }

  Ok(lines)
}

fn render_slide_preview(lines: &[String]) -> DynamicImage {
  let mut image =
    ImageBuffer::from_pixel(PREVIEW_WIDTH, PREVIEW_HEIGHT, Rgba([231, 245, 255, 255]));

  fill_rect(
    &mut image,
    PADDING,
    PADDING,
    PREVIEW_WIDTH - (PADDING * 2),
    PREVIEW_HEIGHT - (PADDING * 2),
    Rgba([255, 255, 255, 255]),
  );

  draw_border(
    &mut image,
    PADDING,
    PADDING,
    PREVIEW_WIDTH - (PADDING * 2),
    PREVIEW_HEIGHT - (PADDING * 2),
    Rgba([186, 230, 253, 255]),
  );

  let chars_per_line = ((PREVIEW_WIDTH - (PADDING * 2) - 48) / 9) as usize;
  let wrapped = wrap_lines(lines, chars_per_line, MAX_LINES);

  let mut y = PADDING + 26;
  for (index, line) in wrapped.iter().enumerate() {
    let color = if index == 0 {
      Rgba([15, 23, 42, 255])
    } else {
      Rgba([30, 41, 59, 255])
    };

    draw_text_line(&mut image, PADDING + 20, y, line, color);
    y += LINE_HEIGHT;

    if y + LINE_HEIGHT >= PREVIEW_HEIGHT - PADDING {
      break;
    }
  }

  DynamicImage::ImageRgba8(image)
}

fn wrap_lines(lines: &[String], width: usize, max_lines: usize) -> Vec<String> {
  let mut wrapped = Vec::new();

  for paragraph in lines {
    let mut line = String::new();

    for word in paragraph.split_whitespace() {
      if line.is_empty() {
        line.push_str(word);
      } else if line.len() + 1 + word.len() <= width {
        line.push(' ');
        line.push_str(word);
      } else {
        wrapped.push(line);
        line = word.to_string();
      }

      if wrapped.len() >= max_lines {
        return wrapped;
      }
    }

    if !line.is_empty() {
      wrapped.push(line);
    }

    if wrapped.len() >= max_lines {
      return wrapped;
    }

    wrapped.push(String::new());
  }

  if wrapped.is_empty() {
    wrapped.push("(empty presentation)".to_string());
  }

  wrapped.truncate(max_lines);
  wrapped
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
    if cursor_x + 8 >= PREVIEW_WIDTH - PADDING {
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
