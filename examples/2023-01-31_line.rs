use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::gradient::{grad_many, lerp};
use nannou_sketches::utilities::u32_to_srgba;

fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    location: Vec2,
    color: Alpha<Rgb<f32>, f32>,
    color_t: f64,
    noise_t: (f64, f64),
}

impl Point {
    fn new(location: Vec2, color: Alpha<Rgb<f32>, f32>) -> Self {
        Self {
            location,
            color,
            color_t: random_range(-500.0, 500.0),
            noise_t: (random_range(-500.0, 500.0), random_range(-500.0, 500.0)),
        }
    }
}

struct Line {
    points: Vec<Point>,
    color_t: f64,
}

impl Line {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            color_t: random_range(-500.0, 500.0),
        }
    }
    fn _update(&mut self) {}
    fn draw(&self, draw: &Draw) {
        let points = self.points.iter().map(|p| (p.location, p.color));
        draw.polyline().weight(1.0).points_colored(points);
    }
}

struct Model {
    points: Vec<Vec2>,
    lines: Vec<Line>,
    colors: Vec<Alpha<Rgb<f32>, f32>>,
    perlin: Perlin,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62]
        .iter()
        .map(|c| u32_to_srgba(*c, 0.25))
        .collect();
    let mut points = Vec::new();
    let mut lines = Vec::new();
    let mut counter = 0.0;
    let radius = 300.0;
    let num_points = 48.0;
    while counter < 2.0 * PI {
        let x = counter.cos() * radius;
        let y = counter.sin() * radius;
        points.push(pt2(x, y));
        counter += (2.0 * PI) / num_points;
    }
    let mut connections: Vec<(Vec2, Vec2)> = Vec::new();
    for p1 in &points {
        for p2 in &points {
            if p1 != p2 && !connections.contains(&(*p2, *p1)) {
                counter = 0.0;
                let mut line = Line::new();
                while counter <= 1.01 {
                    let x = lerp(p1.x, p2.x, counter);
                    let y = lerp(p1.y, p2.y, counter);
                    line.points
                        .push(Point::new(pt2(x, y), srgba(1.0, 1.0, 1.0, 1.0)));
                    counter += 0.05;
                }
                lines.push(line);
                connections.push((*p1, *p2));
            }
        }
    }
    Model {
        points,
        lines,
        colors,
        perlin: Perlin::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = pt2(app.mouse.x, app.mouse.y);
    for line in &mut model.lines {
        let cnoise = model.perlin.get([line.color_t, 0.0]) as f32;
        let cnoise = map_range(cnoise, -1.0, 1.0, 0.0, 1.0);
        for point in &mut line.points {
            if point.location.distance(mouse) <= 25.0 {
                let angle =
                    (point.location.x - mouse.x).atan2(point.location.y - mouse.y) * (180.0 / PI);
                let x = mouse.x + angle.cos() * 25.0;
                let y = mouse.y + angle.sin() * 25.0;
                point.location.x = x;
                point.location.y = y;
            }
            //let noise_x = model.perlin.get([point.noise_t.0, 0.0]) as f32*0.5;
            //let noise_y = model.perlin.get([point.noise_t.1, 0.0]) as f32*0.5;
            //point.location.x += noise_x;
            //point.location.y += noise_y;
            //point.noise_t.0 += 0.0001;
            //point.noise_t.1 += 0.0001;
            //let cnoise = model.perlin.get([point.color_t, 0.0]) as f32;
            //let cnoise = map_range(cnoise, -1.0, 1.0, 0.0, 1.0);
            point.color = grad_many(&model.colors, cnoise);
            //point.color_t += 0.05;
        }
        line.color_t += 0.04;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for line in &model.lines {
        line.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
