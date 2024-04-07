#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod av1encoder;
mod error;
pub use av1encoder::Encoder;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str) {
    println!("{}", s);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn avif_from_imagedata(
    rgba: &[u8],
    width: usize,
    height: usize,
    quality: f32,
    speed: u8,
) -> Vec<u8> {
    Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .encode_rgb(width, height, rgba.chunks_exact(4))
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::Encoder;
    use std::fs::File;
    use std::io::{BufReader, Write};

    #[test]
    fn test_jellyfish_small() {
        let input = File::open("jellyfish_small.jpg").unwrap();
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(input));
        let rgb = decoder.decode().unwrap();
        let width = decoder.info().unwrap().width as usize;
        let height = decoder.info().unwrap().height as usize;
        let quality = 50.0;
        let speed = 6;
        let bytes = Encoder::new()
            .with_quality(quality)
            .with_speed(speed)
            .encode_rgb(width, height, rgb.chunks_exact(3))
            .unwrap();
        let mut output = File::options()
            .create(true)
            .write(true)
            .open("jellyfish_small.avif")
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
}
