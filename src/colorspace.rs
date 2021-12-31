fn srgb2rgb(color: &[u8; 3]) -> [f32; 3] {
    fn srgb2rgb_channel(channel: u8) -> f32 {
        let c = channel as f32 / 255.;
        let c = if c > 0.04045 {
            ((c + 0.055) / 1.055).powf(2.4)
        } else {
            c / 12.92
        };
        255. * c
    }
    color.map(srgb2rgb_channel)
}

fn rgb2srgb(color: &[f32; 3]) -> [f32; 3] {
    fn rgb2srgb_channel(channel: f32) -> f32 {
        let c = channel / 255.;
        let c = if c < 0.0031308 {
            c * 12.92
        } else {
            1.055 * c.powf(1. / 2.4) - 0.055
        };
        255. * c
    }
    color.map(rgb2srgb_channel)
}

pub fn mix_colors(color_1: &[u8; 3], color_2: &[u8; 3]) -> [u8; 3] {
    // Convert to linear rgb space
    let color_1 = srgb2rgb(color_1);
    let color_2 = srgb2rgb(color_2);
    let srgb_mixed_color = [
        (color_1[0] + color_2[0]) / 2.,
        (color_1[1] + color_2[1]) / 2.,
        (color_1[2] + color_2[2]) / 2.,
    ];
    let rgb_mixed_color = rgb2srgb(&srgb_mixed_color);
    [
        rgb_mixed_color[0].round() as u8,
        rgb_mixed_color[1].round() as u8,
        rgb_mixed_color[2].round() as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::mix_colors;
    #[test]
    fn colorspace() {
        let color = [12, 25, 65];
        let new_color = mix_colors(&color, &color);
        assert_eq!(color[0], new_color[0]);
        assert_eq!(color[1], new_color[1]);
        assert_eq!(color[2], new_color[2]);
    }
}
