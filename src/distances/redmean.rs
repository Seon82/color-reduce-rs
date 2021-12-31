pub fn dist_redmean_squared(color_1: &[u8; 3], color_2: &[u8; 3]) -> f32 {
    let r_mean = (color_1[0] + color_2[0]) as f32 / 2.;
    let dr = color_1[0] as f32 - color_2[0] as f32;
    let dg = color_1[1] as f32 - color_2[1] as f32;
    let db = color_1[2] as f32 - color_2[2] as f32;
    (2. + r_mean / 256.) * dr.powi(2) + 4. * dg.powi(2) + (2. + (255. - r_mean) / 256.) * db.powi(2)
}

pub fn dist_redmean(color_1: &[u8; 3], color_2: &[u8; 3]) -> f32 {
    dist_redmean_squared(color_1, color_2).sqrt()
}
