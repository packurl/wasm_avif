use wasm_bindgen::prelude::*;
mod av1encoder;
mod error;
pub use av1encoder::Encoder;
use rgb::{AsPixels, RGBA8};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn avif_from_imagedata(
    rgba: &[u8],
    width: usize,
    height: usize,
    quality: f32,
    speed: u8,
) -> Vec<u8> {
    let res = Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .encode_rgb(rgba.as_pixels(), width, height)
        .map_err(|e| log(format!("{:?}", e).as_str()))
        .unwrap();
    res.avif_file
}
