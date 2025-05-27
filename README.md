[WASM](https://developer.mozilla.org/en-US/docs/WebAssembly) libs for [avif](https://aomediacodec.github.io/av1-avif/) image compression.

This is a fork of the [ravif](https://github.com/kornelski/cavif-rs/tree/main/ravif) [rust](https://www.rust-lang.org/) [crate](https://crates.io/crates/ravif).

<br>

Compilation:

`wasm-pack build --target web`

<br>

Dependencies:
- [avif-serialize](https://github.com/kornelski/avif-serialize) ([BSD3 License](https://github.com/kornelski/avif-serialize/blob/main/LICENSE))
- [rav1e](https://github.com/xiph/rav1e) ([BSD2 License](https://github.com/xiph/rav1e/blob/master/LICENSE))
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) ([MIT License](https://github.com/r

<br>

---

```
jellyfish.jpg 
jpg : 876454
avif: 157994, speed: 10,  time:  35.1
avif: 137079, speed:  9,  time:  39.6
avif: 137217, speed:  8,  time:  70.9
avif: 137217, speed:  7,  time:  70.7
avif: 136226, speed:  6,  time:  78.9
avif: 136226, speed:  5,  time:  79.2
avif: 136110, speed:  4,  time:  79.8
avif: 136235, speed:  3,  time:  81.2
avif: 132943, speed:  2,  time:  96.4
avif: 121904, speed:  1,  time: 523.4

jellyfish_small.jpg
jpg : 13964
avif:  6818, speed: 10,  time: 0.6
avif:  6264, speed:  9,  time: 0.6
avif:  6229, speed:  8,  time: 1.0
avif:  6229, speed:  7,  time: 1.0
avif:  6198, speed:  6,  time: 1.1
avif:  6198, speed:  5,  time: 1.1
avif:  6179, speed:  4,  time: 1.2
avif:  6198, speed:  3,  time: 1.2
avif:  6027, speed:  2,  time: 1.6
avif:  5852, speed:  1,  time: 7.8
```
