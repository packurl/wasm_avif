#[derive(Debug)]
#[doc(hidden)]
pub struct EncodingErrorDetail; // maybe later

#[derive(Debug)]
pub enum Error {
    TooFewPixels,
    Unsupported(&'static str),
    EncodingError(String),
}
