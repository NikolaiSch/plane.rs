use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeaderErrors {
    #[error("Failed to parse")]
    RequestError,
    #[error("Invalid compression algorithm: {0}")]
    EncodingError(String),
    #[error("Invalid HTTP version: {0}")]
    HTTPVersionError(String),
    #[error("Invalid Locale: {0}")]
    LocaleError(String),
    #[error("Invalid Method: {0}")]
    MethodError(String),
    #[error("Invalid or Unsupported Mime type: {0}")]
    MimeTypeError(String)
}
