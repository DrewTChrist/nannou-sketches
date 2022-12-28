//! This was an interesting problem to solve. I initially tried to
//! keep one value in the model struct to increment and get the sin value.
//! Looping through all the points in a wave in the update function
//! was incrementing the step to quickly to do anything nice. I ended up
//! giving the Line struct a `sin` value. This number is the incremented
//! number that I get sine from. When instantiating each Line struct, I 
//! increment the `sin` value so that it's different for each line but
//! only by a small increment.
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
            .weight(1.0);
    }
}

type SinWave = Vec<Line>;

struct Model {
    wave_1: SinWave,
    wave_2: SinWave,
    wave_3: SinWave,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    // Make sure the directory where we will save images to exists.
    let bounds = app.window_rect();
    let mut wave_1 = Vec::<Line>::new();
    let mut wave_2 = Vec::<Line>::new();
    let mut wave_3 = Vec::<Line>::new();
    let mut x: f32 = bounds.left();
    let mut sin: f32 = 0.0;
    while x <= bounds.right() {
        wave_1.push(Line::new(pt2(x, 0.0), pt2(x, 0.0), sin));
        wave_2.push(Line::new(pt2(x + 1.0, 0.0), pt2(x + 1.0, 0.0), sin));
        wave_3.push(Line::new(pt2(x + 2.0, 0.0), pt2(x + 2.0, 0.0), sin));
        x += 10.0;
        sin += 0.1;
    }

    Model {
        wave_1,
        wave_2,
        wave_3,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    for line in model.wave_1.iter_mut() {
        let sin = map_range(line.sin.sin(), -1.0, 1.0, 0.0, bounds.top());
        line.start.y = -sin;
        line.end.y = sin;
        line.sin += 0.015;
    }
    for line in model.wave_2.iter_mut() {
        let sin = map_range(line.sin.sin(), -1.0, 1.0, 0.0, bounds.top());
        line.start.y = -sin;
        line.end.y = sin;
        line.sin += 0.015;
    }
    for line in model.wave_3.iter_mut() {
        let sin = map_range(line.sin.sin(), -1.0, 1.0, 0.0, bounds.top());
        line.start.y = -sin;
        line.end.y = sin;
        line.sin += 0.015;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .w_h(WIDTH as f32, HEIGHT as f32)
        .color(rgba(0.0, 0.0, 0.0, 0.1));

    for line in &model.wave_1 {
        line.draw(&draw, WHITE);
    }

    for line in &model.wave_2 {
        line.draw(&draw, BLUE);
    }

    for line in &model.wave_3 {
        line.draw(&draw, RED); }
    draw.to_frame(app, &frame).unwrap();
}
