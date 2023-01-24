use nannou::color::Alpha;
use nannou::prelude::*;

pub fn u32_to_srgba(color: u32, alpha: f32) -> Alpha<Rgb<f32>, f32> {
    let r = color >> 16;
    let g = color >> 8 & 0x00ff;
    let b = color & 0x0000ff;
    srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, alpha)
}
