use crate::distances::dist_cie2000;
use crate::mix_colors;
use lab::Lab;
use std::collections::HashMap;

#[derive(Clone)]
pub struct BasePalette {
    pub rgb_colors: Vec<[u8; 3]>,
    pub lab_colors: Vec<Lab>,
}

impl BasePalette {
    pub fn new(rgb_colors: Vec<[u8; 3]>) -> BasePalette {
        let lab_colors = rgb_colors.iter().map(Lab::from_rgb).collect();
        BasePalette {
            rgb_colors,
            lab_colors,
        }
    }
}

pub struct ExtendedPalette {
    pub base_colors: BasePalette,
    pub mixed_colors: BasePalette,
    pub rgb_mixes: HashMap<[u8; 3], [usize; 2]>,
}
impl ExtendedPalette {
    pub fn from_base(base_palette: &BasePalette, dither_threshold: u8) -> ExtendedPalette {
        let mut rgb_mixes = HashMap::new();
        let mut mixed_rgb_colors = Vec::new();
        let dither_threshold = 120. * dither_threshold as f32 / 255.; // ciede2000 maxes out at ~120.
        for c1_idx in 0..base_palette.rgb_colors.len() {
            for c2_idx in c1_idx..base_palette.rgb_colors.len() {
                let c1_lab = &base_palette.lab_colors[c1_idx];
                let c2_lab = &base_palette.lab_colors[c2_idx];
                if dist_cie2000(c1_lab, c2_lab) <= dither_threshold {
                    let mix = mix_colors(
                        &base_palette.rgb_colors[c1_idx],
                        &base_palette.rgb_colors[c2_idx],
                    );
                    mixed_rgb_colors.push(mix);
                    // Put the lighter color first
                    if c1_lab.l >= c2_lab.l {
                        rgb_mixes.insert(mix, [c1_idx, c2_idx]);
                    } else {
                        rgb_mixes.insert(mix, [c2_idx, c1_idx]);
                    }
                }
            }
        }
        ExtendedPalette {
            base_colors: base_palette.clone(),
            mixed_colors: BasePalette::new(mixed_rgb_colors),
            rgb_mixes,
        }
    }
}
