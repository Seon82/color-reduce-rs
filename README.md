# color-reduce

A rust library to map images to a given palette, which is typically useful for pixel art, or for artists wishing to create a retro effect.

## Installation
Use this library in your project by adding ``color-reduce : {git: "https://github.com/Seon82/color-reduce"}`` to your ``Cargo.toml`` dependencies.

## Quickstart

```rust
use color_reduce::{QuantizeMethod, quantize};

fn main() {
    let mut img = image::open("path/to/image").unwrap().to_rgb8();
    let palette = get_palette();
    // Modify the buffer in place
    quantize(&mut img, &palette, QuantizeMethod::CIE2000, Some(255));
    img.save("output/image/path").unwrap();
}

fn get_palette() -> BasePalette {
    BasePalette::new(vec![
        [0, 0, 0],
        [34, 34, 34],
        [85, 85, 85],
        [136, 136, 136],
        [205, 205, 205],
        [255, 255, 255],
    ])
}

```