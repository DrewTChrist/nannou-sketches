use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::grids::{hex_grid, hex_grid_test};

fn main() {
    nannou::app(model).update(update).run();
}

struct Line {
    points: Vec<Vec2>,
    color: Alpha<Rgb<f32>, f32>,
    len: usize,
}

impl Line {
    fn new(x: f32, y: f32, color: Alpha<Rgb<f32>, f32>) -> Self {
        let mut points = Vec::new();
        points.push(pt2(x, y));
        Self {
            points,
            color,
            len: 1,
        }
    }
    fn update(&mut self) {
        if self.points.len() > 5 {
            self.points.remove(0);
        }
    }
    fn push(&mut self, point: Vec2) {
        if self.len < 500 {
            self.points.push(point);
            self.len += 1;
        }
    }
    fn draw(&self, draw: &Draw) {
        //for point in &self.points {
        //    draw.rect()
        //        .xy(*point)
        //        .w_h(10.0, 10.0)
        //        .color(self.color);
        //}
        for i in 1..self.points.len() {
            draw.line()
                .start(self.points[i - 1])
                .end(self.points[i])
                .weight(1.0)
                .color(self.color);
        }
    }
}

struct Point {
    location: Vec2,
    angle: f32,
}

impl Point {
    fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            location: pt2(x, y),
            angle,
        }
    }
}

struct Model {
    //grid: Vec<Vec2>,
    grid: Vec<Point>,
    lines: Vec<Line>,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut grid = Vec::new();
    hex_grid_test(&mut grid, 50.0, -20.0, -20.0, 30, 30);
    let mut point_grid = Vec::new();
    let perlin = Perlin::new();
    let mut tx = random_range(-500.0, 500.0);
    let mut ty = random_range(-500.0, 500.0);
    for xy in grid {
        //let noise = perlin.get([t, 0.0]);
        let noise = perlin.get([tx + (xy.x as f64 * 0.005), ty + (xy.y as f64 * 0.005)]);
        let mapped = map_range(noise, -1.0, 1.0, -2.0 * PI, 2.0 * PI);
        point_grid.push(Point::new(xy.x, xy.y, mapped));
        tx += 0.001;
        ty += 0.001;
    }
    let mut line_points = Vec::new();
    let mut lines = Vec::new();
    for _ in 0..3000 {
        let x = random_range(-300.0, 300.0);
        let y = random_range(-300.0, 300.0);
        if !line_points.contains(&pt2(x, y)) {
            line_points.push(pt2(x, y));
            lines.push(Line::new(x, y, srgba(0.0, 0.0, 1.0, 1.0)));
        }
    }
    Model {
        grid: point_grid,
        lines,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //for point in &model.grid {
    //    for line in &mut model.lines {
    //        let line_point = line.points[line.points.len() - 1];
    //        if point.location.distance(line_point) <= 10.0 {
    //            let x = line_point.x + point.angle.cos();
    //            let y = line_point.y + point.angle.sin();
    //            line.push(pt2(x, y));
    //        }
    //        if line.points.len() > 5 {
    //            line.points.remove(0);
    //        }
    //    }
    //}
    for line in &mut model.lines {
        let line_point = line.points[line.points.len() - 1];
        let mut distance = f32::MAX;
        let mut p = &Point::new(0.0, 0.0, 0.0);
        for point in &model.grid {
            let point_dist = point.location.distance(line_point);
            if point_dist <= distance {
                distance = point_dist;
                p = &point;
            }
        }
        let x = line_point.x + p.angle.cos();
        let y = line_point.y + p.angle.sin();
        line.push(pt2(x, y));
        if line.points.len() > 5 {
            line.points.remove(0);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //draw.background().color(BLACK);
    //for point in &model.grid {
    //    draw.ellipse()
    //        .xy(point.location)
    //        .radius(1.0)
    //        .color(WHITE);
    //    let x = point.location.x + point.angle.cos() * 5.0;
    //    let y = point.location.y + point.angle.sin() * 5.0;
    //    draw.line()
    //        .start(point.location)
    //        .end(pt2(x, y))
    //        .weight(1.0)
    //        .color(RED);
    //}
    for line in &model.lines {
        //for i in 1..line.points.len() {
        for i in 0..line.points.len() {
            let dist = map_range(pt2(0.0, 0.0).distance(line.points[i]), 0.0, 300.0, 1.0, 0.0);
            draw.ellipse()
                .xy(line.points[i])
                .radius(0.5)
                .color(srgba(dist, dist, dist, 0.1));
            //draw.line()
            //    .start(line.points[i - 1])
            //    .end(line.points[i])
            //    .weight(0.1)
            //    //.weight(5.0 - (dist / 300.0) * 5.0)
            //    //.color(line.color);
            //    .color(WHITE);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
