use image::ImageError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GenthumbError {
  UnsupportedFormat(String),
  IoError(io::Error),
  ImageError(ImageError),
}

impl fmt::Display for GenthumbError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      GenthumbError::UnsupportedFormat(s) => write!(f, "Unsupported file format: {}", s),
      GenthumbError::IoError(e) => write!(f, "IO error: {}", e),
      GenthumbError::ImageError(e) => write!(f, "Image error: {}", e),
    }
  }
}

impl std::error::Error for GenthumbError {}

impl From<io::Error> for GenthumbError {
  fn from(err: io::Error) -> Self {
    GenthumbError::IoError(err)
  }
}

impl From<ImageError> for GenthumbError {
  fn from(err: ImageError) -> Self {
    GenthumbError::ImageError(err)
  }
}

impl GenthumbError {
  pub fn is_unsupported_format(&self) -> bool {
    matches!(self, GenthumbError::UnsupportedFormat(_))
  }
}

impl From<GenthumbError> for napi::Error {
  fn from(err: GenthumbError) -> Self {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  }
}
