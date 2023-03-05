use avif::Encoder;
use imgref::Img;
use rgb::AsPixels;
use std::fs;
use std::time::Instant;

fn main() {
    let width = 1703;
    let height = 2560;
    let pixels = fs::read("./data.bin").unwrap();
    // let mut data = fs::read("./jellyfish.jpg").unwrap();
    // let mut decoder = jpeg_decoder::Decoder::new(data.as_slice());
    // let pixels = decoder.decode().unwrap();
    println!("{}", pixels.len());
    // let meta = decoder.info().unwrap();
    // let width = meta.width as usize;
    // let height = meta.height as usize;
    let t = Instant::now();
    let res = Encoder::new()
        .with_quality(67.0)
        .with_speed(6)
        .encode_rgb(Img::new(pixels.as_slice().as_pixels(), width, height))
        // .encode_rgb(img.as_ref())
        .unwrap();
    println!("{}", res.avif_file.len());
    println!("{}", res.color_byte_size);
    println!("{}", t.elapsed().as_secs_f32());
    fs::write("./data.avif", res.avif_file).unwrap();
}
