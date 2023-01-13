use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use std::cmp::Ordering::Equal;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<(Vec2, f32, f32)>,
    perlin: Perlin,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(300, 300).view(view).build().unwrap();
    //let window = app.main_window();
    let bounds = app.window_rect();
    let mut points: Vec<(Vec2, f32, f32)> = Vec::new();
    let perlin = Perlin::new();
    for _ in 0..15 {
        points.push((
            pt2(
                random_range(bounds.left(), bounds.right()),
                random_range(bounds.bottom(), bounds.top()),
            ),
            random_range(-500.0, 500.0),
            random_range(-500.0, 500.0),
        ));
    }
    Model { points, perlin }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    for (point, xstep, ystep) in &mut model.points {
        let x_noise = map_range(model.perlin.get([*xstep as f64, 0.0]), -1.0, 1.0, -5.0, 5.0);
        let y_noise = map_range(model.perlin.get([*ystep as f64, 0.0]), -1.0, 1.0, -5.0, 5.0);
        if point.x > bounds.left() && point.x < bounds.right() {
            point.x += x_noise;
        }
        if point.y > bounds.bottom() && point.y < bounds.top() {
            point.y += y_noise;
        }
        *xstep += 0.0001;
        *ystep += 0.0001;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);
    //if app.elapsed_frames() == 1 {
    //    draw.background().color(BLACK);
    //}
    //draw.rect()
    //    .x_y(0.0, 0.0)
    //    .w_h(bounds.w(), bounds.h())
    //    .color(srgba(0.0, 0.0, 0.0, 0.1));
    let n = 0;
    //for x in (bounds.left() as i32..bounds.right() as i32 + 1).step_by(15) {
    //    for y in (bounds.bottom() as i32..bounds.top() as i32 + 1).step_by(15) {
    for x in (bounds.left() as i32..bounds.right() as i32 + 1).step_by(10) {
        for y in (bounds.bottom() as i32..bounds.top() as i32 + 1).step_by(10) {
            let mut distances = Vec::new();
            for (point, _, _) in &model.points {
                let dist = pt2(x as f32, y as f32).distance(*point);
                distances.push(dist);
            }
            distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
            let noise = map_range(distances[n], distances[0], distances[distances.len() -1], 255.0, 0.0);
            draw.ellipse()
                .resolution(5.0)
                .x_y(x as f32, y as f32)
                .radius(map_range(distances[n], 0.0, distances[distances.len() -1] * 0.5, 5.0, 0.0))
                .color(srgba(
                    noise as f32 / 255.0,
                    noise as f32 / 255.0,
                    noise as f32 / 255.0,
                    1.0,
                ));
        }
    }
    for (point, _, _) in &model.points {
        draw.ellipse()
            .resolution(10.0)
            .x_y(point.x, point.y).radius(5.0).color(RED);
    }
    draw.to_frame(app, &frame).unwrap();
}
