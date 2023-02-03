use nannou::color::Alpha;
use nannou::ease;
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
    points: Vec<Circle>,
    rand_points: Vec<Vec2>,
    t: f32,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut points = Vec::new();
    let mut rand_points = Vec::new();
    let mut step = 0.0;
    let mut swirl_radius = 0.0;
    let mut circle_radius = 1.0;
    let mut color = 0.0;
    let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62]
        .iter()
        .map(|c| u32_to_srgba(*c, 0.7))
        .collect();
    let mut color_t = 0.0;
    while step <= 48.0 * PI {
        let x = step.cos() * swirl_radius;
        let y = step.sin() * swirl_radius;
        points.push(Circle {
            location: pt2(x, y),
            radius: circle_radius,
            color: grad_many(&colors, color_t),
        });
        step += 0.25;
        //swirl_radius += 0.75;
        //circle_radius += 0.05;
        circle_radius += 0.05;
        swirl_radius = circle_radius * 10.0;
        color += 0.00135;
        color_t += 0.0015;
    }
    println!("{:#?}", &points[0..10]);
    for i in 0..points.len() {
        rand_points.push(pt2(
            random_range(-300.0, 300.0),
            random_range(-300.0, 300.0),
        ));
    }
    Model {
        points,
        rand_points,
        t: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for idx in 0..model.points.len() {
        let change_x = model.points[idx].location.x - model.rand_points[idx].x;
        let change_y = model.points[idx].location.y - model.rand_points[idx].y;
        let x = ease::quad::ease_in(model.t, model.rand_points[idx].x, change_x, 1.0);
        let y = ease::quad::ease_in(model.t, model.rand_points[idx].y, change_y, 1.0);
        model.rand_points[idx].x = x;
        model.rand_points[idx].y = y;
    }
    model.t += 0.003;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let draw = app.draw();
    let c = BEIGE;
    let bg = srgba(
        c.red as f32 / 255.0,
        c.green as f32 / 255.0,
        c.blue as f32 / 255.0,
        0.1,
    );
    if app.elapsed_frames() == 0 {
        draw.background().color(c);
    }
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(bounds.w(), bounds.h())
        .color(bg);
    for idx in 0..model.points.len() {
        draw.ellipse()
            .resolution(14.0)
            .xy(model.rand_points[idx])
            .radius(model.points[idx].radius)
            .color(model.points[idx].color);
    }
    draw.to_frame(app, &frame).unwrap();
}
