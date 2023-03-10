use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::utilities::u32_to_srgba;

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
    points: Vec<Point>,
    perlin: Perlin,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let colors = vec![
        //0xffd4b2, 0xfff6bd, 0xceedc7, 0x86c8bc,
        //0x65647c, 0x8b7e74, 0xc7bca1, 0xf1d3b3
        0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62,
    ];
    let mut points = Vec::new();
    let radius = 1.0;
    //let mut xnoise = random_range(-500.0, 500.0);
    //let mut ynoise = random_range(-500.0, 500.0);
    let mut xnoise = random_range(-500.0, 500.0);
    let mut ynoise = xnoise + random_range(0.0, 50.0);
    for _ in 0..500 {
        points.push(Point {
            location: pt2(0.0, 0.0),
            radius,
            noise: (xnoise, ynoise),
            color: u32_to_srgba(colors[random_range(0, colors.len())], 1.0),
            resolution: 10.0,
        });
        xnoise += 0.1;
        ynoise += 0.1;
    }
    let perlin = Perlin::new();
    Model { points, perlin }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    //let tx = map_range(app.time.cos(), -1.0, 1.0, 0.0, 10.0);
    //let ty = map_range(app.time.sin(), -1.0, 1.0, 0.0, 10.0);
    //let tx = app.time.cos() * 10.0;
    //let ty = app.time.sin() * 10.0;
    let a = 6.0;
    let b = 12.0;
    let h = 6.0;
    let t = app.time;
    let tx = (a - b) * t.cos() + h * (((a - b) / b) * t).cos();
    let ty = (a - b) * t.sin() + h * (((a - b) / b) * t).sin();
    for point in &mut model.points {
        let xnoise = model.perlin.get([point.noise.0, 0.0]);
        let ynoise = model.perlin.get([point.noise.1, 0.0]);
        let xnmap = map_range(xnoise, -1.0, 1.0, bounds.left(), bounds.right());
        let ynmap = map_range(ynoise, -1.0, 1.0, bounds.bottom(), bounds.top());
        point.location.x = xnmap as f32;
        point.location.y = ynmap as f32;
        point.noise.0 += tx as f64 * 0.00025;
        point.noise.1 += ty as f64 * 0.00025;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(DARKSLATEGRAY);
    }
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(bounds.w(), bounds.h())
        .color(srgba(
            DARKSLATEGRAY.red as f32 / 255.0,
            DARKSLATEGRAY.blue as f32 / 255.0,
            DARKSLATEGRAY.green as f32 / 255.0,
            0.05,
        ));
    for point in &model.points {
        draw.ellipse()
            .xy(point.location)
            .radius(point.radius)
            //.resolution(point.resolution)
            .color(point.color);
    }
    draw.to_frame(app, &frame).unwrap();
}
