use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("int parsing error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("error processing image")]
    ImageProcessingError(#[from] image::ImageError),
    #[error("failed loading image {0}")]
    ImageOpenFailed(String),
    #[error("failed to fetch index {0}[{1}]")]
    InvalidArrayIndex(String, String),
    #[error("Is missing required data. Please use the RoboHashBuilder.")]
    RoboHashMissingRequiredData,
    #[error("unknown error")]
    Unknown,
}
