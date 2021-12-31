mod colorspace;
pub mod distances;
pub mod palette;

pub use colorspace::mix_colors;

use distances::{dist_cie2000_squared, dist_luma, dist_redmean_squared};
use image::{Rgb, RgbImage};
use lab::Lab;
pub use palette::{BasePalette, ExtendedPalette};
use std::collections::HashMap;

pub enum QuantizeMethod {
    CIE2000,
    Redmean,
    Luma,
}

pub fn quantize(
    img: &mut RgbImage,
    palette: &BasePalette,
    method: QuantizeMethod,
    dither_threshold: Option<u8>,
) {
    match dither_threshold {
        Some(threshold) => quantize_dither(img, palette, method, threshold),
        None => quantize_no_dither(img, palette, method),
    }
}
fn quantize_no_dither(img: &mut RgbImage, palette: &BasePalette, method: QuantizeMethod) {
    let mut color_map = HashMap::new();
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let pixel_colors = [pixel[0], pixel[1], pixel[2]];
        let new_colors = match color_map.get(&pixel_colors) {
            Some(quantized_color) => *quantized_color, // Great news, we've already calculated it!
            None => {
                // Oh well, let's do the expensive computation
                let quantized_color = match method {
                    QuantizeMethod::CIE2000 => get_nearest_ciede(&pixel_colors, palette),
                    QuantizeMethod::Redmean => get_nearest_redmean(&pixel_colors, palette),
                    QuantizeMethod::Luma => get_nearest_luma(&pixel_colors, palette),
                };
                color_map.insert(pixel_colors, quantized_color);
                quantized_color
            }
        };
        *pixel = Rgb(new_colors);
    }
}

fn quantize_dither(
    img: &mut RgbImage,
    palette: &BasePalette,
    method: QuantizeMethod,
    dither_threshold: u8,
) {
    let palette = ExtendedPalette::from_base(palette, dither_threshold);
    println!("{:?}", palette.mixed_colors.rgb_colors.len());
    let mut color_map = HashMap::new();
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let pixel_colors = [pixel[0], pixel[1], pixel[2]];
        let new_colors = match color_map.get(&pixel_colors) {
            Some(quantized_color) => *quantized_color, // Great news, we've already calculated it!
            None => {
                // Oh well, let's do the expensive computation
                let quantized_color = match method {
                    QuantizeMethod::CIE2000 => {
                        get_nearest_ciede(&pixel_colors, &palette.mixed_colors)
                    }
                    QuantizeMethod::Redmean => {
                        get_nearest_redmean(&pixel_colors, &palette.mixed_colors)
                    }
                    QuantizeMethod::Luma => get_nearest_luma(&pixel_colors, &palette.mixed_colors),
                };
                color_map.insert(pixel_colors, quantized_color);
                quantized_color
            }
        };
        let new_colors = match palette.rgb_mixes.get(&new_colors) {
            Some(color_idx) => {
                palette.base_colors.rgb_colors[color_idx[((y as u32 + x as u32) % 2) as usize]]
            }
            None => new_colors,
        };
        *pixel = Rgb(new_colors);
    }
}
// Find the index of the point in point_cloud that's closest to point using the dist distance function
fn get_closest_idx<T>(point: &T, point_cloud: &[T], dist: fn(&T, &T) -> f32) -> usize {
    let mut min_dist: f32 = std::f32::INFINITY;
    let mut closest_idx: usize = 0;

    for (i, cloud_point) in point_cloud.iter().enumerate() {
        let dist = dist(point, cloud_point);

        if dist < min_dist {
            min_dist = dist;
            closest_idx = i;
        }
    }
    closest_idx
}

fn get_nearest_ciede(pixel: &[u8; 3], palette: &BasePalette) -> [u8; 3] {
    let pixel = Lab::from_rgb(pixel);
    let idx = get_closest_idx(&pixel, &palette.lab_colors, dist_cie2000_squared);

    palette.rgb_colors[idx]
}

fn get_nearest_redmean(pixel: &[u8; 3], palette: &BasePalette) -> [u8; 3] {
    let idx = get_closest_idx(pixel, &palette.rgb_colors, dist_redmean_squared);

    palette.rgb_colors[idx]
}

fn get_nearest_luma(pixel: &[u8; 3], palette: &BasePalette) -> [u8; 3] {
    let idx = get_closest_idx(pixel, &palette.rgb_colors, dist_luma);

    palette.rgb_colors[idx]
}
