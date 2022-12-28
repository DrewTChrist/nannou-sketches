use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

const HEIGHT: u32 = 600;
const WIDTH: u32 = 1000;

fn main() {
    nannou::app(model).update(update).run();
}

struct Line {
    start: Vec2,
    end: Vec2,
    sin: f32,
}

impl Line {
    fn new(start: Vec2, end: Vec2, sin: f32) -> Self {
        Self { start, end, sin }
    }

    fn draw(&self, draw: &Draw, color: Rgb<u8>) {
        draw.line()
            .start(self.start)
            .end(self.end)
            .color(color)
            .weight(2.5);
    }
}

struct Model {
    lines: Vec<Line>,
    lines2: Vec<Line>,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    let bounds = app.window_rect();
    let mut lines = Vec::<Line>::new();
    let mut lines2 = Vec::<Line>::new();
    let mut x: f32 = bounds.left();
    let mut sin: f32 = 0.0;
    while x <= bounds.right() {
        lines.push(Line::new(pt2(x, 0.0), pt2(x, 0.0), sin));
        lines2.push(Line::new(pt2(x + 1.0, 0.0), pt2(x + 1.0, 0.0), sin));
        x += 10.0;
        sin += 0.1;
    }
    Model {
        lines,
        lines2
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    for line in model.lines.iter_mut() {
        let sin = map_range(line.sin.sin(), -1.0, 1.0, 0.0, bounds.top());
        line.start.y = -sin;
        line.end.y = sin;
        //model.sin += 0.75;
        line.sin += 0.025;
    }
    for line in model.lines2.iter_mut() {
        let sin = map_range(line.sin.sin(), -1.0, 1.0, 0.0, bounds.top());
        line.start.y = -sin;
        line.end.y = sin;
        //model.sin += 0.75;
        line.sin += 0.025;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //draw.background().color(BLACK);

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .w_h(WIDTH as f32, HEIGHT as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for line in &model.lines {
        line.draw(&draw, BLUE);
    }

    for line in &model.lines2 {
        line.draw(&draw, RED);
    }

    draw.to_frame(app, &frame).unwrap();
}
