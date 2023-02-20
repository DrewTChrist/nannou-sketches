use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Vec<Vec2>,
    start: usize,
    end: usize,
}

macro_rules! grid_points {
    (width=$w:expr, height=$h:expr, scale=$scale:expr, step=$step:expr) => {{
        let mut grid = Vec::new();
        let grid_w: isize = $w as isize / $scale;
        let grid_h: isize = $h as isize / $scale;
        for i in (-grid_w + $step..grid_w).step_by($step) {
            for j in (-grid_h + $step..grid_h).step_by($step) {
                grid.push(pt2(i as f32, j as f32));
            }
        }
        grid
    }};
}

fn model(app: &App) -> Model {
    let height = 600;
    let width = 600;
    let _window_id = app
        .new_window()
        .size(height, width)
        .view(view)
        .build()
        .unwrap();

    let grid = grid_points!(width = 600, height = 600, scale = 2, step = 15);
    let len = grid.len();

    Model {
        grid,
        start: random_range(0, len),
        end: random_range(0, len),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //let start = random_range(0, model.grid.len());
    //let end = random_range(0, model.grid.len());
    model.start = model.end;
    model.end = random_range(0, model.grid.len());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //draw.background().color(BLACK);
    //for point in &model.grid {
    //    draw.ellipse().xy(*point).radius(2.0).color(WHITE);
    //}
    draw.line()
        .start(model.grid[model.start])
        .end(model.grid[model.end])
        .color(WHITE)
        .weight(2.5);
    draw.to_frame(app, &frame).unwrap();
}
