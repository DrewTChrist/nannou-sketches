use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn model(_app: &App) -> Model {
    Model {
        x: 0.0,
        y: 75.0,
        vx: 0.0,
        vy: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let elapsed_time = app.duration.since_prev_update.as_secs_f32();

    // Apply gravity to the vertical velocity.
    model.vy -= 9.8 * elapsed_time * 20.0;

    // Update the position based on the velocity.
    model.x += model.vx * elapsed_time;
    model.y += model.vy * elapsed_time;

    // If the ball hits the ground, bounce it back up with a reduced velocity.
    if model.y < 0.0 {
        model.y = 0.0;
        model.vy = -model.vy * 0.8;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // Draw the ball.
    draw.ellipse()
        .x_y(model.x, model.y)
        .radius(10.0)
        .color(WHITE);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
