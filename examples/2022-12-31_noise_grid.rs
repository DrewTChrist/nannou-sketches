use nannou::prelude::*;
use nannou_sketches::grids::NoiseGrid;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grids: Vec<NoiseGrid>,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut grids = Vec::<NoiseGrid>::new();
    for i in (-300..300).step_by(150) {
        for j in (-300..300).step_by(150) {
            let ng = NoiseGrid::new(pt2(i as f32, j as f32), 50, 50, 2.0);
            grids.push(ng);
        }
    }
    Model { grids }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for grid in model.grids.iter_mut() {
        grid.update_colors();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for grid in &model.grids {
        grid.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
