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
    //points: Vec<Vec2>
    points: Vec<Point>,
    perlin: Perlin,
    frame: bool,
}

/*fn u32_to_srgba(color: u32, alpha: f32) -> Alpha<Rgb<f32>, f32> {
    let r = color >> 16;
    let g = color >> 8 & 0x00ff;
    let b = color & 0x0000ff;
    srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, alpha)
}*/

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut points = Vec::new();
    let mut t = 0.0;
    let radius = 1.0;
    let colors = vec![
        0xffd4b2, 0xfff6bd, 0xceedc7,
        0x86c8bc,
        //0x65647c, 0x8b7e74, 0xc7bca1, 0xf1d3b3
        //0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62,
    ];
    for _ in 0..600 {
        points.push(Point {
            location: pt2(t.cos() * radius, t.sin() * radius),
            radius: 1.0,
            noise: (random_range(-500.0, 500.0), random_range(-500.0, 500.0)),
            color: u32_to_srgba(
                colors[random_range(0, colors.len())],
                random_range(0.0, 1.0),
            ),
            resolution: 5.0,
        });
        t += 0.05;
    }
    let perlin = Perlin::new();
    Model {
        points,
        perlin,
        frame: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for point in &mut model.points {
        point.location.x *= 1.005;
        point.location.y *= 1.005;
        point.radius += 0.01;
        point.resolution = map_range(point.noise.0.sin(), -1.0, 1.0, 1.0, 10.0); //random_range(1.0, 10.0);
        point.noise.0 += 0.01;
        point.noise.1 += 0.01;
    }
    if app.time / 5.0 > 10.51 {
        model.frame = true;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = app.time / 5.0;
    let bounds = app.window_rect();
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(DARKSLATEGRAY);
    }
    draw.color_blend(BLEND_SUBTRACT);
    for point in &model.points {
        let xnoise = model.perlin.get([point.noise.0, 0.0]);
        let ynoise = model.perlin.get([point.noise.1, 0.0]);
        let xnmap = map_range(xnoise, -1.0, 1.0, -15.0, 15.0);
        let ynmap = map_range(ynoise, -1.0, 1.0, -15.0, 15.0);
        if point.location.distance(pt2(0.0, 0.0)) <= 250.0 {
            draw.ellipse()
                .x_y(
                    point.location.x + xnmap as f32,
                    point.location.y + ynmap as f32,
                )
                .radius(point.radius)
                .resolution(point.resolution)
                .color(point.color);
        } else {
            let x = t.cos() * 260.0;
            let y = t.sin() * 260.0;
            draw.ellipse().x_y(x, y).radius(20.0).color(DARKSLATEGRAY);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if t > 10.5 && model.frame == false {
        app.main_window().capture_frame("frame3.png");
    }
}
