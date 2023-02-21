use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};

fn main() {
    nannou::app(model).update(update).run();
}

macro_rules! grid_points {
    (width=$w:expr, height=$h:expr, scale=$scale:expr, step=$step:expr) => {{
        let mut grid = Vec::new();
        let grid_w: isize = $w as isize / $scale;
        let grid_h: isize = $h as isize / $scale;
        for i in (-grid_w + $step..grid_w).step_by($step) {
            for j in (-grid_h + $step..grid_h).step_by($step) {
                grid.push(pt2(j as f32, i as f32));
            }
        }
        grid
    }};
}

struct Model {
    grid: Vec<Vec2>,
    start: usize,
    end: usize,
    perlin: Perlin,
    t: f64
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

    let mut grid = grid_points!(width = 600, height = 600, scale = 2, step = 5);
    let len = grid.len();

    Model {
        grid,
        start: random_range(0, len),
        end: random_range(0, len),
        perlin: Perlin::new(),
        t: random_range(-500.0, 500.0)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //let start = random_range(0, model.grid.len());
    //let end = random_range(0, model.grid.len());
    let noise = model.perlin.get([model.t, 0.0]);
    let nmap = map_range(noise, -1.0, 1.0, 0.0, model.grid.len() as f32);
    model.start = model.end;
    //model.end = random_range(0, model.grid.len());
    //model.end = nmap as usize;
    let mut distance = 1000000.0;
    let mut point = &pt2(0.0, 0.0);
    for point in &model.grid {
        if model.end.distance(point) <= distance {
            distance = model.end.distance(point);
            point = &point;
        }
    }
    model.t += 0.0001;
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
