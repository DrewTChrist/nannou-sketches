use nannou::color::Alpha;
use nannou::prelude::*;

pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

pub fn gradient(
    c1: Alpha<Rgb<f32>, f32>,
    c2: Alpha<Rgb<f32>, f32>,
    mut t: f32,
) -> Alpha<Rgb<f32>, f32> {
    t = clamp(t, 0.0, 1.0);
    let r = lerp(c1.red, c2.red, t);
    let g = lerp(c1.green, c2.green, t);
    let b = lerp(c1.blue, c2.blue, t);
    let a = lerp(c1.alpha, c2.alpha, t);
    srgba(r, g, b, a)
}

pub fn gradient_mut(
    c: &mut Alpha<Rgb<f32>, f32>,
    c1: Alpha<Rgb<f32>, f32>,
    c2: Alpha<Rgb<f32>, f32>,
    mut t: f32,
) {
    t = clamp(t, 0.0, 1.0);
    c.red = lerp(c1.red, c2.red, t);
    c.green = lerp(c1.green, c2.green, t);
    c.blue = lerp(c1.blue, c2.blue, t);
    c.alpha = lerp(c1.alpha, c2.alpha, t);
}

pub fn grad_many(cs: &Vec<Alpha<Rgb<f32>, f32>>, mut t: f32) -> Alpha<Rgb<f32>, f32> {
    t = clamp(t, 0.0, 1.0);
    let num_colors = cs.len() as f32;
    let step = 1.0 / (num_colors - 1.0);
    let mut greatest = (0.0, 0, 0);
    for i in 0..cs.len() - 1 {
        let val = i as f32 * step;
        if t >= val {
            greatest = (val, i, i + 1);
        }
    }
    let start_color = cs[greatest.1];
    let end_color = cs[greatest.2];
    gradient(
        start_color,
        end_color,
        (t * (num_colors - 1.0)) - greatest.1 as f32,
    )
}
