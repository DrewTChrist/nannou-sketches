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
    /*points: Vec<Circle>,
    t: f32,*/
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    /*let mut points = Vec::new();
    let mut step = 0.0;
    let mut swirl_radius = 1.0;
    let mut circle_radius = 5.0;
    let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62]
        .iter()
        .map(|c| u32_to_srgba(*c, 1.0))
        .collect();
    let mut color_t = 0.0;
    while step <= 12.0 * PI {
        let x = step.cos() * swirl_radius;
        let y = step.sin() * swirl_radius;
        points.push(Circle {
            location: pt2(x, y),
            radius: circle_radius,
            color: grad_many(&colors, color_t),
        });
        //step += 0.25;
        step += 0.25;
        //circle_radius += 0.05;
        circle_radius += 0.1;
        //swirl_radius = circle_radius * 10.0;
        swirl_radius += 1.0;
        color_t += 0.0015;
    }*/
    //Model { points, t: 0.0 }
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let _bounds = app.window_rect();
    let draw = app.draw();
    let c = BEIGE;
    let bg = srgba(
        c.red as f32 / 255.0,
        c.green as f32 / 255.0,
        c.blue as f32 / 255.0,
        0.1,
    );
    draw.background().color(bg);
    let mut points = Vec::new();
    let mut step = 0.0;
    let mut swirl_radius = 1.0;
    let mut circle_radius = 5.0;
    let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62]
        .iter()
        .map(|c| u32_to_srgba(*c, 1.0))
        .collect();
    let mut color_t = 0.0;
    while step <= 12.0 * PI {
        let x = step.cos() * swirl_radius;
        let y = step.sin() * swirl_radius;
        points.push(Circle {
            location: pt2(x, y),
            radius: circle_radius,
            color: grad_many(&colors, color_t),
        });
        //step += 0.25;
        step += 0.25;
        //circle_radius += 0.05;
        circle_radius += 0.1;
        //swirl_radius = circle_radius * 10.0;
        swirl_radius += 1.0;
        color_t += 0.0015;
    }
    //for point in &model.points {
    for point in points {
        draw.ellipse()
            .xy(point.location)
            .color(point.color)
            .radius(point.radius);
    }
    draw.to_frame(app, &frame).unwrap();
}
