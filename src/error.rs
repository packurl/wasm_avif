#[derive(Debug)]
pub enum Error {
    TooFewPixels,
    Unsupported(&'static str),
    EncodingError(String),
}
