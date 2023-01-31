use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::utilities::u32_to_srgba;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    colors: Vec<Alpha<Rgb<f32>, f32>>,
    pos: Vec2,
    t: f32,
    perlin: Perlin,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let colors = vec![
        u32_to_srgba(0xa7d2cb, 1.0),
        u32_to_srgba(0xf2d388, 1.0),
        u32_to_srgba(0xc98474, 1.0),
        u32_to_srgba(0x874c62, 1.0),
    ];
    let perlin = Perlin::new();
    Model {
        colors,
        pos: pt2(-250.0, 0.0),
        t: 0.0,
        perlin,
    }
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

fn gradient(
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

fn gradient_mut(
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

fn grad_many(cs: &Vec<Alpha<Rgb<f32>, f32>>, mut t: f32) -> Alpha<Rgb<f32>, f32> {
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

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.t += 0.005;
    model.pos.x += 2.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let bounds = app.window_rect();
    //draw.background().color(BLACK);
    draw.ellipse()
        .x_y(model.pos.x, model.pos.y)
        .radius(50.0)
        //.color(gradient(model.colors[0], model.colors[2], model.t));
        .color(grad_many(&model.colors, model.t));
    draw.to_frame(app, &frame).unwrap();
}
