use nannou::color::Alpha;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn texture_circle(
    x: f32,
    y: f32,
    d: f32,
    fg: Alpha<Rgb<f32>, f32>,
    bg: Alpha<Rgb<f32>, f32>,
    draw: &Draw,
) {
    let cap = d as u32 / 2 * 100;
    draw.ellipse().x_y(x, y).radius(d / 2.0).color(bg);
    for i in 0..cap {
        let theta1 = random_range(0.0, 1.0) * 2.0 * PI;
        let x1 = x + theta1.cos() * d / 2.0;
        let y1 = y + theta1.sin() * d / 2.0;
        let theta2 = random_range(0.0, 1.0) * 2.0 * PI;
        let x2 = x + theta2.cos() * d / 2.0;
        let y2 = y + theta2.sin() * d / 2.0;
        draw.line()
            .start(pt2(x1, y1))
            .end(pt2(x2, y2))
            .color(fg)
            .weight(0.005);
    }
}

struct Model {
    point: Vec2,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    Model {
        point: pt2(0.0, 0.0),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    //draw.background().color(BLACK);
    let c = DARKSLATEGRAY;
    let fg = srgba(1.0, 1.0, 1.0, 0.1);
    let bg = srgba(
        c.red as f32 / 255.0,
        c.green as f32 / 255.0,
        c.blue as f32 / 255.0,
        1.0,
    );
    if app.elapsed_frames() == 1 {
        texture_circle(0.0, 0.0, 300.0, fg, bg, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
