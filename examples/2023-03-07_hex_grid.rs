use nannou::prelude::*;
use nannou_sketches::grids::hex_grid;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Vec<Vec2>,
    index: usize,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut grid = Vec::new();
    hex_grid(&mut grid, 0, 0, 25, 25, 10.0);
    Model { grid, index: 0 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.grid = Vec::new();
    hex_grid(&mut model.grid, 0, 0, 25, 25, app.mouse.x);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for point in &model.grid {
        draw.ellipse().xy(*point).radius(1.0).color(RED);
    }
    draw.to_frame(app, &frame).unwrap();
}
