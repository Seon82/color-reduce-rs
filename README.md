# color-reduce

A rust library to map images to a given palette, which is typically useful for pixel art, or for artists wishing to create a retro effect.

## Installation
Use this library in your project by adding ``color-reduce : {git: "https://github.com/Seon82/color-reduce"}`` to your ``Cargo.toml`` dependencies.

## Quickstart

```rust
use color_reduce;
use color_reduce::{quantize, BasePalette, QuantizeMethod};

fn main() {
    let mut img = image::open("path/to/image").unwrap().to_rgb8();
    // Palette used on pxls.space
    let palette = BasePalette::pxls();
    // Modify the buffer in place
    quantize(&mut img, &palette, QuantizeMethod::CIE2000, Some(255));
    img.save("output/image/path").unwrap();
}
```