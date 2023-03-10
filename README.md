[WASM](https://developer.mozilla.org/en-US/docs/WebAssembly) libs for [avif](https://aomediacodec.github.io/av1-avif/) image compression.

This is a fork of the [ravif](https://github.com/kornelski/cavif-rs/tree/main/ravif) [rust](https://www.rust-lang.org/) [crate](https://crates.io/crates/ravif).

<br>

Compilation:

`wasm-pack build --target web`

<br>

Dependencies:
- [avif-serialize](https://github.com/kornelski/avif-serialize) ([BSD3 License](https://github.com/kornelski/avif-serialize/blob/main/LICENSE))
- [rav1e](https://github.com/xiph/rav1e) ([BSD2 License](https://github.com/xiph/rav1e/blob/master/LICENSE))
- [rgb](https://github.com/kornelski/rust-rgb) ([MIT License](https://github.com/kornelski/rust-rgb/blob/main/LICENSE))
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) ([MIT License](https://github.com/r
