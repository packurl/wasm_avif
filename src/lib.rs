use wasm_bindgen::prelude::*;
mod av1encoder;
mod error;
pub use av1encoder::Encoder;
use imgref::Img;
use rgb::AsPixels;

#[wasm_bindgen]
pub fn avif_from_imagedata(
    rgba: &[u8],
    width: usize,
    height: usize,
    quality: f32,
    speed: u8,
) -> Vec<u8> {
    // return rgba.to_vec();
    let res = Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .encode_rgb(Img::new(rgba.as_pixels(), width, height))
        .unwrap();
    res.avif_file
}
