use nannou::color::Alpha;
use nannou::geom::Rect;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn noise_line(
    mut start: Vec2,
    mut end: Vec2,
    color: Alpha<Rgb<f32>, f32>,
    draw: &Draw,
    noise: &Perlin,
) {
    let mut tmp: f32;
    if start.x > end.x {
        tmp = start.x;
        start.x = end.x;
        end.x = tmp;
        tmp = start.y;
        start.y = end.y;
        end.y = tmp;
    }

    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let mut step = 1.0;

    if end.x < start.x {
        step = -step;
    }
    let mut sx = start.x;
    let mut sy = start.y;
    let mut x = start.x + step;
    while x <= end.x {
        let y = start.y + step * dy * (x - start.x) / dx;
        let snoise = noise.get([sx as f64 * 0.1, sy as f64 * 0.1]);
        let weight = 1.0 + map_range(snoise, -1.0, 1.0, -0.25, 0.25);
        draw.line()
            .start(pt2(sx, sy))
            .end(pt2(
                x + noise.get([x as f64 * 0.1, y as f64 * 0.1]) as f32,
                y + noise.get([x as f64 * 0.1, y as f64 * 0.1]) as f32,
            ))
            .weight(weight)
            .color(color);
        x += step;
        sx = x;
        sy = y;
    }
}

struct Model {
    noise: Perlin,
    colors: Vec<Alpha<Rgb<f32>, f32>>,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut colors = Vec::<Alpha<Rgb<f32>, f32>>::new();
    for i in 0..(3600 / 5) {
        let g = random_range(0.0, 1.0);
        let a = random_range(0.1, 0.5);
        colors.push(srgba(g, g, g, a));
    }
    Model {
        noise: Perlin::new(),
        colors,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    //let bounds = app.window_rect();
    let bounds = Rect::from_x_y_w_h(0.0, 0.0, 50.0, 50.0);
    let draw = app.draw();
    draw.background().color(BLACK);
    let spacing = 5.0;
    {
        let mut counter = 0;
        let mut i = -bounds.w();
        while i < bounds.h() + bounds.w() {
            noise_line(
                pt2(i as f32, 0.0),
                pt2(i as f32 + bounds.h(), bounds.h()),
                model.colors[counter],
                &draw,
                &model.noise,
            );
            i += spacing;
            counter += 1;
        }
    }
    {
        let mut counter = 0;
        let mut i = bounds.h() + bounds.w();
        while i >= -bounds.w() {
            noise_line(
                pt2(i as f32, 0.0),
                pt2(i as f32 - bounds.h(), bounds.h()),
                model.colors[counter],
                &draw,
                &model.noise,
            );
            i -= spacing;
            counter += 1;
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
