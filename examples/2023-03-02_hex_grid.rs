use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn hex_grid(grid: &mut Vec<Vec2>, radius: f32) {
    let mut inner_grid = Vec::new();
    for i in (-300..300).step_by(25) {
        for j in (-300..300).step_by(25) {
            inner_grid.push(pt2(i as f32, j as f32));
        }
    }
}

struct Model {
    grid: Vec<Vec2>,
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(600, 600)
        .view(view)
        .build()
        .unwrap();
    let mut grid = Vec::new();
    
    Model { grid }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let rows = 3;
    let cols = 4;
    let top = 150.0;
    let radius = 50.0;
    let mut point = pt2(-150.0, 150.0);
    for i in 0..cols {
        if i % 2 == 0 {
            point.y = top - radius;
        } else {
            point.y = top;
        }
        for _ in 0..rows {
            draw.ellipse()
                .xy(point)
                .radius(2.5)
                //.radius(50.0)
                .color(WHITE);
            /*let mut t = 0.0;
            for _ in 0..6 {
                let x = point.x + t.cos() * radius;
                let y = point.y + t.sin() * radius;
                draw.ellipse()
                    .x_y(x, y)
                    .radius(2.5)
                    .color(RED);
                t += (2.0 * PI) / 6.0;
            }*/
            point.y -= radius * 2.0;
        }
        point.x += radius * 1.5;
    }
    draw.to_frame(app, &frame).unwrap();
}
/*
let mut t = 0.0;
for i in 0..6 {
    let x = t.cos() * 100.0;
    let y = t.sin() * 100.0;
    draw.ellipse()
        .x_y(x, y)
        .radius(10.0)
        .color(WHITE);
    t += (2.0 * PI) / 6.0;
}
*/
