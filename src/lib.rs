use wasm_bindgen::prelude::*;
mod av1encoder;
mod error;
pub use av1encoder::Encoder;

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
    hdr: bool,
    quality: f32,
    speed: u8,
) -> Vec<u8> {
    Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .encode_rgb(
            width,
            height,
            if hdr { 10u8 } else { 8u8 },
            rgba.chunks_exact(4),
        )
        .map_err(|e| log(format!("{:?}", e).as_str()))
        .unwrap()
}
