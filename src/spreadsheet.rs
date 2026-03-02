use anyhow::{Context, Result, anyhow};
use calamine::{Reader, open_workbook_auto};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

const MAX_ROWS: usize = 18;
const MAX_COLS: usize = 6;
const CELL_WIDTH: u32 = 180;
const CELL_HEIGHT: u32 = 32;
const PADDING: u32 = 8;

pub fn render_preview(path: &Path) -> Result<DynamicImage> {
  let rows = match path
    .extension()
    .and_then(|extension| extension.to_str())
    .map(|extension| extension.to_ascii_lowercase())
  {
    Some(extension) if extension == "csv" || extension == "tsv" => read_delimited(path)?,
    _ => read_workbook(path)?,
  };

  Ok(render_grid(&rows))
}

fn read_delimited(path: &Path) -> Result<Vec<Vec<String>>> {
  let delimiter = match path
    .extension()
    .and_then(|extension| extension.to_str())
    .map(|extension| extension.to_ascii_lowercase())
    .as_deref()
  {
    Some("tsv") => b'\t',
    _ => b',',
  };

  let mut reader = csv::ReaderBuilder::new()
    .has_headers(false)
    .delimiter(delimiter)
    .from_path(path)
    .with_context(|| format!("Failed to read delimited file: {}", path.display()))?;

  let mut rows = Vec::new();

  for record in reader.records().take(MAX_ROWS) {
    let record = record.with_context(|| format!("Failed to parse rows in: {}", path.display()))?;
    let row = record
      .iter()
      .take(MAX_COLS)
      .map(sanitize_cell)
      .collect::<Vec<_>>();
    rows.push(row);
  }

  if rows.is_empty() {
    rows.push(vec!["(empty spreadsheet)".to_string()]);
  }

  Ok(rows)
}

fn read_workbook(path: &Path) -> Result<Vec<Vec<String>>> {
  let mut workbook = open_workbook_auto(path)
    .with_context(|| format!("Failed to open workbook: {}", path.display()))?;

  let range = workbook
    .worksheet_range_at(0)
    .ok_or_else(|| anyhow!("Spreadsheet has no worksheets"))?
    .with_context(|| format!("Failed to read first worksheet in: {}", path.display()))?;

  let mut rows = Vec::new();

  for row in range.rows().take(MAX_ROWS) {
    let values = row
      .iter()
      .take(MAX_COLS)
      .map(|value| sanitize_cell(&value.to_string()))
      .collect::<Vec<_>>();
    rows.push(values);
  }

  if rows.is_empty() {
    rows.push(vec!["(empty spreadsheet)".to_string()]);
  }

  Ok(rows)
}

fn render_grid(rows: &[Vec<String>]) -> DynamicImage {
  let row_count = rows.len().max(1) as u32;
  let col_count = rows.iter().map(|row| row.len()).max().unwrap_or(1).max(1) as u32;

  let width = (col_count * CELL_WIDTH) + (PADDING * 2) + 1;
  let height = (row_count * CELL_HEIGHT) + (PADDING * 2) + 1;

  let mut image = ImageBuffer::from_pixel(width, height, Rgba([248, 250, 252, 255]));

  draw_grid_lines(&mut image, col_count, row_count);

  for (row_idx, row) in rows.iter().enumerate() {
    for col_idx in 0..col_count as usize {
      let x = PADDING + (col_idx as u32 * CELL_WIDTH);
      let y = PADDING + (row_idx as u32 * CELL_HEIGHT);

      if row_idx == 0 {
        fill_rect(
          &mut image,
          x + 1,
          y + 1,
          CELL_WIDTH.saturating_sub(1),
          CELL_HEIGHT.saturating_sub(1),
          Rgba([224, 242, 254, 255]),
        );
      }

      let value = row.get(col_idx).map_or("", String::as_str);
      let text = ellipsize(value, max_chars_per_cell());

      draw_text(
        &mut image,
        x + 6,
        y + 12,
        &text,
        Rgba([17, 24, 39, 255]),
        CELL_WIDTH.saturating_sub(12),
      );
    }
  }

  DynamicImage::ImageRgba8(image)
}

fn draw_grid_lines(image: &mut RgbaImage, col_count: u32, row_count: u32) {
  let color = Rgba([191, 219, 254, 255]);

  for col in 0..=col_count {
    let x = PADDING + (col * CELL_WIDTH);
    let y_start = PADDING;
    let y_end = PADDING + (row_count * CELL_HEIGHT);
    draw_vertical_line(image, x, y_start, y_end, color);
  }

  for row in 0..=row_count {
    let y = PADDING + (row * CELL_HEIGHT);
    let x_start = PADDING;
    let x_end = PADDING + (col_count * CELL_WIDTH);
    draw_horizontal_line(image, x_start, x_end, y, color);
  }
}

fn draw_horizontal_line(image: &mut RgbaImage, x_start: u32, x_end: u32, y: u32, color: Rgba<u8>) {
  for x in x_start..=x_end {
    if x < image.width() && y < image.height() {
      image.put_pixel(x, y, color);
    }
  }
}

fn draw_vertical_line(image: &mut RgbaImage, x: u32, y_start: u32, y_end: u32, color: Rgba<u8>) {
  for y in y_start..=y_end {
    if x < image.width() && y < image.height() {
      image.put_pixel(x, y, color);
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

fn draw_text(image: &mut RgbaImage, x: u32, y: u32, text: &str, color: Rgba<u8>, max_width: u32) {
  let mut cursor_x = x;
  let max_x = x + max_width;

  for character in text.chars() {
    if cursor_x + 8 > max_x {
      break;
    }

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
  }
}

fn sanitize_cell(value: &str) -> String {
  value
    .replace(['\r', '\n'], " ")
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
}

fn ellipsize(value: &str, max_chars: usize) -> String {
  let chars = value.chars().collect::<Vec<_>>();

  if chars.len() <= max_chars {
    return value.to_string();
  }

  let keep = max_chars.saturating_sub(1);
  let shortened = chars.into_iter().take(keep).collect::<String>();
  format!("{}…", shortened)
}

fn max_chars_per_cell() -> usize {
  ((CELL_WIDTH.saturating_sub(12)) / 9) as usize
}
