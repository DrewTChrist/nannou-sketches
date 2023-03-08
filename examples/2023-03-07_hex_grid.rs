use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::gradient::lerp;
use nannou_sketches::grids::hex_grid;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Vec<Vec2>,
    random: Vec2,
    noise_x: f64,
    noise_y: f64,
    perlin: Perlin,
}

const RADIUS: f32 = 10.0;

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut grid = Vec::new();
    hex_grid(&mut grid, -15, -15, 30, 30, RADIUS);
    //let x = random_range(-300.0, 300.0);
    //let y = random_range(-300.0, 300.0);
    let x = 0.0;
    let y = 0.0;
    Model {
        grid,
        random: pt2(x, y),
        noise_x: random_range(-500.0, 500.0),
        noise_y: random_range(-500.0, 500.0),
        perlin: Perlin::new()
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    //model.grid = Vec::new();
    //hex_grid(&mut model.grid, app.mouse.x as isize, app.mouse.y as isize, 25, 25, 15.0);
    let mut distance = f32::MAX;
    let mut p = pt2(0.0, 0.0);
    for point in &model.grid {
        let dist = point.distance(model.random);
        if dist <= distance {
            distance = dist;
            p = *point;
        }
        //draw.ellipse().xy(*point).radius(1.0).color(RED);
    }
    let xn = model.perlin.get([model.noise_x, 0.0]);
    let yn = model.perlin.get([model.noise_y, 0.0]);
    let xnm = map_range(xn, -1.0, 1.0, -RADIUS*2.0, RADIUS*2.0);
    let ynm = map_range(yn, -1.0, 1.0, -RADIUS*2.0, RADIUS*2.0);
    model.random.x = lerp(model.random.x, p.x, 0.5) + xnm;
    model.random.y = lerp(model.random.y, p.y, 0.5) + ynm;
    model.noise_x += 0.05;
    model.noise_y += 0.05;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //draw.background().color(BLACK);
    //for point in &model.grid {
    //    draw.ellipse().xy(*point).radius(1.0).color(RED);
    //}
    draw.ellipse().xy(model.random).radius(1.0).color(BLUE);
    draw.to_frame(app, &frame).unwrap();
}
