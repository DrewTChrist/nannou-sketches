cat > examples/$(date -I)_$1.rs <<- EOF
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(600, 600)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
EOF
