// Formula originally used created by CtrlZ's for pxls.fiddle
pub fn dist_luma(color_1: &[u8; 3], color_2: &[u8; 3]) -> f32 {
    let (r1, g1, b1) = (color_1[0] as f32, color_1[1] as f32, color_1[2] as f32);
    let (r2, g2, b2) = (color_2[0] as f32, color_2[1] as f32, color_2[2] as f32);
    let dr = (r1 - r2) / 255.;
    let dg = (g1 - g2) / 255.;
    let db = (b1 - b2) / 255.;
    // Calculate luma using Rec. 601
    let l1 = r1 * 0.299 + g1 * 0.587 + b1 * 0.114;
    let l2 = r2 * 0.299 + g2 * 0.587 + b2 * 0.114;
    let dl = (l1 - l2) / 255.;

    (dr.powi(2) * 0.299 + dg.powi(2) * 0.587 + db.powi(2) * 0.114) * 0.75 + dl.powi(2)
}
