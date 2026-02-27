use image::ImageError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum SipatError {
  UnsupportedFormat(String),
  IoError(io::Error),
  ImageError(ImageError),
}

impl fmt::Display for SipatError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SipatError::UnsupportedFormat(s) => write!(f, "Unsupported file format: {}", s),
      SipatError::IoError(e) => write!(f, "IO error: {}", e),
      SipatError::ImageError(e) => write!(f, "Image error: {}", e),
    }
  }
}

impl std::error::Error for SipatError {}

impl From<io::Error> for SipatError {
  fn from(err: io::Error) -> Self {
    SipatError::IoError(err)
  }
}

impl From<ImageError> for SipatError {
  fn from(err: ImageError) -> Self {
    SipatError::ImageError(err)
  }
}

impl SipatError {
  pub fn is_unsupported_format(&self) -> bool {
    matches!(self, SipatError::UnsupportedFormat(_))
  }
}

impl From<SipatError> for napi::Error {
  fn from(err: SipatError) -> Self {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  }
}
