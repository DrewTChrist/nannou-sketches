cat > examples/$1.rs <<- EOF
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        // event or update
        //.event(event)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

// can do event or update
//fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}
EOF
