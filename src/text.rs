use anyhow::{Context, Result};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use std::fs;
use std::path::Path;

const PREVIEW_WIDTH: u32 = 1100;
const PREVIEW_HEIGHT: u32 = 1500;
const PADDING: u32 = 36;
const LINE_HEIGHT: u32 = 16;
const MAX_LINES: usize = 90;

pub fn render_preview(path: &Path) -> Result<DynamicImage> {
  let text = fs::read_to_string(path)
    .with_context(|| format!("Failed to read text file: {}", path.display()))?;

  Ok(render_text_preview(&text))
}

fn render_text_preview(content: &str) -> DynamicImage {
  let mut image =
    ImageBuffer::from_pixel(PREVIEW_WIDTH, PREVIEW_HEIGHT, Rgba([248, 250, 252, 255]));

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
    Rgba([203, 213, 225, 255]),
  );

  let chars_per_line = ((PREVIEW_WIDTH - (PADDING * 2) - 32) / 9) as usize;
  let lines = wrap_text(content, chars_per_line, MAX_LINES);

  let mut y = PADDING + 20;
  for line in lines {
    draw_text_line(&mut image, PADDING + 16, y, &line, Rgba([30, 41, 59, 255]));
    y += LINE_HEIGHT;

    if y + LINE_HEIGHT >= PREVIEW_HEIGHT - PADDING {
      break;
    }
  }

  DynamicImage::ImageRgba8(image)
}

fn wrap_text(content: &str, width: usize, max_lines: usize) -> Vec<String> {
  let normalized = content.replace('\r', "");
  let mut lines = Vec::new();

  for paragraph in normalized.lines() {
    if paragraph.trim().is_empty() {
      lines.push(String::new());
      if lines.len() >= max_lines {
        return lines;
      }
      continue;
    }

    let mut current = String::new();
    for word in paragraph.split_whitespace() {
      if current.is_empty() {
        current.push_str(word);
      } else if current.len() + 1 + word.len() <= width {
        current.push(' ');
        current.push_str(word);
      } else {
        lines.push(current);
        current = word.to_string();
      }

      if lines.len() >= max_lines {
        return lines;
      }
    }

    if !current.is_empty() {
      lines.push(current);
      if lines.len() >= max_lines {
        return lines;
      }
    }
  }

  if lines.is_empty() {
    lines.push("(empty text file)".to_string());
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
