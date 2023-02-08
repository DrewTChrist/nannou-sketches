use nannou::color::Alpha;
//use nannou::ease;
use nannou::prelude::*;
use nannou_sketches::{gradient::grad_many, utilities::u32_to_srgba};

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct Circle {
    location: Vec2,
    radius: f32,
    color: Alpha<Rgb<f32>, f32>,
}

struct Model {
    colors: Vec<Alpha<Rgb<f32>, f32>>,
    color_t: Vec<f32>
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62, 0xa7d2cb]
        .iter()
        .map(|c| u32_to_srgba(*c, 1.0))
        .collect();
    let mut color_t = Vec::new();
    let mut t = 0.0;
    let mut ct = 0.0;
    while t < 12.0 * PI {
        color_t.push(ct);
        t += 0.1;
        ct += 0.002;
    }
    Model { colors, color_t }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.color_t.len() {
        if model.color_t[i] < 1.0 {
            model.color_t[i] += 0.05;
        } else {
            model.color_t[i] = 0.0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _bounds = app.window_rect();
    let draw = app.draw();
    draw.background().color(BEIGE);
    //let t_step = 0.25;
    let t_step = 0.1;
    let circ_rad_step = 0.1;
    //let swirl_rad_step = 1.0;
    let swirl_rad_step = 0.5;
    let color_t_step = 0.002;
    let range = 12.0 * PI;
    let mut t = 0.0;
    let mut swirl_radius = 1.0;
    let mut circle_radius = 5.0;
    let mut color_t = 0.0;
    //let mut delta = 0.0;
    let mut index: usize = 0;
    while t <= range {
        let x = t.cos() * swirl_radius;
        let y = t.sin() * swirl_radius;
        draw.ellipse()
            .xy(pt2(x, y))
            //.color(grad_many(&model.colors, model.color_t + delta))
            .color(grad_many(&model.colors, model.color_t[index]))
            .radius(circle_radius);
        t += t_step;
        circle_radius += circ_rad_step;
        swirl_radius += swirl_rad_step;
        //color_t += color_t_step;
        //model.color_t += color_t_step;
        //delta += 0.0025;
        index += 1;
    }
    draw.to_frame(app, &frame).unwrap();
}
