#[derive(Debug)]
pub enum Error {
    TooFewPixels,
    Unsupported(&'static str),
    Encoding(String),
}
