use anyhow::{Context, Result};
use image::DynamicImage;
use pdfium_auto::bind_bundled;
use pdfium_render::prelude::*;
use std::path::Path;

pub fn render_first_page(path: &Path) -> Result<DynamicImage> {
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
