use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    spot: Vec<Vec2>,
}
const RADIUS: f32 = 150.0;

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let perlin = Perlin::new();
    let mut spot = Vec::new();
    let mut ntx = random_range(0.0, 50.0);
    let mut nty = random_range(0.0, 50.0);
    let ntxstart = ntx;
    let ntystart = nty;
    //let mut ntx = 5.0;
    //let mut nty = 5.0;
    let mut t = 0.0;
    while t <= 2.0 * PI {
        let noise_x = map_range(perlin.get([ntx, 0.0]), -1.0, 1.0, 50.0, 150.0);
        let noise_y = map_range(perlin.get([nty, 0.0]), -1.0, 1.0, 50.0, 150.0);
        //let x = t.cos() * RADIUS + noise_x;
        //let y = t.sin() * RADIUS + noise_y;
        let x = t.cos() * noise_x;
        let y = t.sin() * noise_y;
        spot.push(pt2(x, y));
        t += 0.001;
        ntx += 0.001;
        nty += 0.001;
    }
    let noise_x = map_range(perlin.get([ntxstart, 0.0]), -1.0, 1.0, 50.0, 150.0);
    let noise_y = map_range(perlin.get([ntystart, 0.0]), -1.0, 1.0, 50.0, 150.0);
    let x = t.cos() * noise_x;
    let y = t.sin() * noise_y;
    spot.push(pt2(x, y));
    //let l = spot.len() - 1;
    //spot[l] = pt2(x, y);
    Model { spot }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.polyline()
        .weight(5.0)
        .color(WHITE)
        .points(model.spot.clone());
    draw.to_frame(app, &frame).unwrap();
}
