use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

const RADIUS: f32 = 20.0;
const FLOOR: f32 = 0.0;

struct Model {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn model(_app: &App) -> Model {
    Model {
        x: 0.0,
        y: 175.0,
        vx: 0.0,
        vy: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    //let elapsed_time = app.duration.since_prev_update.as_secs_f32();
    let elapsed_time = app.elapsed_frames() as f32 / 60.0;

    model.vy -= 9.8 * elapsed_time;

    model.x += model.vx * elapsed_time;
    model.y += model.vy * elapsed_time;

    if model.y <= FLOOR + RADIUS {
        model.y = FLOOR + RADIUS;
        model.vy = -model.vy * 0.8;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.ellipse()
        .x_y(model.x, model.y)
        .radius(RADIUS)
        .color(WHITE);

    let start = pt2(app.window_rect().left(), FLOOR);
    let end = pt2(app.window_rect().right(), FLOOR);

    draw.line().start(start).end(end).weight(3.0).color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
