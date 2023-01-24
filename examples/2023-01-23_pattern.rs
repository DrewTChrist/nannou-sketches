use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    location: Vec2,
    radius: f32,
    noise: (f64, f64),
    color: Alpha<Rgb<f32>, f32>,
    resolution: f32,
}

struct Model {
    //points: Vec<Vec2>
    points: Vec<Point>,
    perlin: Perlin,
}

fn u32_to_srgba(color: u32, alpha: f32) -> Alpha<Rgb<f32>, f32> {
    let r = color >> 16;
    let g = color >> 8 & 0x00ff;
    let b = color & 0x0000ff;
    srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, alpha)
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let colors = vec![
        //u32_to_srgba(0xffd4b2, 1.0),
        //u32_to_srgba(0xfff6bd, 1.0),
        //u32_to_srgba(0xceedc7, 1.0),
        //u32_to_srgba(0x86c8bc, 1.0),
        //0xffd4b2, 0xfff6bd, 0xceedc7, 0x86c8bc,
        //0x65647c, 0x8b7e74, 0xc7bca1, 0xf1d3b3
        0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62,
    ];
    let mut points = Vec::new();
    let mut t = 0.0;
    let radius = 1.0;
    let mut xnoise = random_range(-500.0, 500.0);
    let mut ynoise = random_range(-500.0, 500.0);
    for _ in 0..600 {
        points.push(Point {
            //location: pt2(t.cos() * radius, t.sin() * radius),
            location: pt2(0.0, 0.0),
            radius: 1.0,
            //noise: (random_range(-500.0, 500.0), random_range(-500.0, 500.0)),
            //noise: (random_range(-0.5, 1.0), random_range(-0.5, 1.0)),
            noise: (xnoise, ynoise),
            color: u32_to_srgba(
                colors[random_range(0, colors.len())],
                1.0
            ),
            resolution: 5.0,
        });
        t += 0.05;
        xnoise += 0.05;
        ynoise += 0.05;
    }
    let perlin = Perlin::new();
    Model { points, perlin }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for point in &mut model.points {
        point.noise.0 += 0.005;
        point.noise.1 += 0.005;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = app.time;
    let bounds = app.window_rect();
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(DARKSLATEGRAY);
    }
    for point in &model.points {
        let xnoise = model.perlin.get([point.noise.0, 0.0]);
        let ynoise = model.perlin.get([point.noise.1, 0.0]);
        let xnmap = map_range(xnoise, -1.0, 1.0, bounds.left(), bounds.right());
        let ynmap = map_range(ynoise, -1.0, 1.0, bounds.bottom(), bounds.top());
        draw.ellipse()
            .x_y(
                point.location.x + xnmap as f32,
                point.location.y + ynmap as f32,
            )
            .radius(point.radius)
            .resolution(point.resolution)
            .color(point.color);
    }
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(bounds.w(), bounds.h())
        .color(srgba(
            DARKSLATEGRAY.red as f32 / 255.0,
            DARKSLATEGRAY.blue as f32 / 255.0,
            DARKSLATEGRAY.green as f32 / 255.0,
            0.1,
        ));
    draw.to_frame(app, &frame).unwrap();
}
