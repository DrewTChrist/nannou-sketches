use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn texture_rect(
    draw: &Draw,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    bg_color: Alpha<Rgb<f32>, f32>,
    fg_color: Alpha<Rgb<f32>, f32>,
) {
    let left = x - (width / 2.0);
    let right = x + (width / 2.0);
    let top = y + (height / 2.0);
    let bottom = y - (height / 2.0);
    let perlin = Perlin::new();
    draw.rect()
        .x_y(x, y)
        .w_h(width, height)
        .color(bg_color);
    for _ in 0..250 {
        let mut x = left;
        let mut y = random_range(bottom, top);
        let mut xt = random_range(-500.0, 500.0);
        let mut yt = random_range(-500.0, 500.0);
        while x < right {
            draw.ellipse().x_y(x, y).radius(0.5).color(fg_color).resolution(3.0);
            let noise = perlin.get([xt, yt]);
            x += 1.0;
            y += noise as f32;
            xt += 0.01;
            yt += 0.01;
        }
    }
}

struct Model {}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 0 {
        draw.background().color(DARKSLATEGRAY);
        texture_rect(
            &draw,
            0.0,
            0.0,
            250.0,
            250.0,
            srgba(0.0, 0.0, 0.0, 1.0),
            srgba(1.0, 1.0, 1.0, 0.05),
        );
    }
    draw.to_frame(app, &frame).unwrap();
}
