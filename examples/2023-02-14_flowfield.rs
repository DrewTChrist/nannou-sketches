use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::utilities::u32_to_srgba;

fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    location: Vec2,
    noise_xt: f64,
    noise_yt: f64,
}

impl Point {
    fn new(x: f32, y: f32, noise_xt: f64, noise_yt: f64) -> Self {
        Self {
            location: pt2(x, y),
            noise_xt,
            noise_yt,
        }
    }
}

struct Line {
    points: Vec<Vec2>,
    color: Alpha<Rgb<f32>, f32>
}

impl Line {
    fn new(x: f32, y: f32, color: Alpha<Rgb<f32>, f32>) -> Self {
        let mut points = Vec::new();
        points.push(pt2(x, y));
        Self { points, color }
    }
    fn update(&mut self) {
        if self.points.len() > 5 {
            self.points.remove(0);
        }
    }
    fn draw(&self, draw: &Draw) {
        for i in 1..self.points.len() {
            draw.line()
                .start(self.points[i - 1])
                .end(self.points[i])
                .weight(10.0)
                .color(self.color);
            }
    }
}

struct Model {
    points: Vec<Point>,
    perlin: Perlin,
    lines: Vec<Line>,
}

impl Model {
    fn draw_grid(&self, draw: &Draw) {
        for point in &self.points {
            let noise = self
                .perlin
                .get([point.noise_xt * 0.5, point.noise_yt * 0.5]);
            let angle = map_range(noise, -1.0, 1.0, -2.0 * PI, 2.0 * PI);
            let x = point.location.x + angle.cos() * 5.0;
            let y = point.location.y + angle.sin() * 5.0;
            draw.line()
                .start(point.location)
                .end(pt2(x, y))
                .weight(1.0)
                .color(RED);
            }
    }
    fn draw_lines(&self, draw: &Draw) {
        for line in &self.lines {
            line.draw(&draw);
        }
    }
    fn update_lines(&mut self) {
        for line in &mut self.lines {
            line.update();
        }
    }
    fn update_grid_noise(&mut self) {
        for point in &mut self.points {
            point.noise_xt += 0.005;
            point.noise_yt += 0.005;
        }
    }
}

fn random_edge(w: f32, h: f32) -> Vec2 {
    let rn = random_range(0.0, 100.0);
    if rn <= 25.0 {
        // top
        pt2(random_range(-w, w), h)
    } else if rn <= 50.0 {
        // right
        pt2(w, random_range(-h, h))
    } else if rn <= 75.0 {
        // bottom
        pt2(random_range(-w, w), -h)
    } else if rn <= 100.0 {
        // left
        pt2(-w, random_range(-h, h))
    } else {
        pt2(0.0, 0.0)
    }
}

const STEP_BY: usize = 10;
const NOISE_T: f64 = 0.0005;
const NUM_LINES: usize = 500;
const DIST: f32 = 15.0;

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut points = Vec::new();
    let mut noise_xt = random_range(-500.0, 500.0);
    let mut noise_yt = random_range(-500.0, 500.0);
    for i in (-250..250).step_by(STEP_BY) {
        for j in (-250..250).step_by(STEP_BY) {
            points.push(Point::new(
                i as f32,
                j as f32,
                noise_xt + i as f64 * 0.01,
                noise_yt + j as f64 * 0.01,
            ));
            noise_xt += NOISE_T;
            noise_yt += NOISE_T;
        }
    }
    let perlin = Perlin::new();
    let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62]
        .iter()
        .map(|c| u32_to_srgba(*c, 0.7))
        .collect();
    let mut lines = Vec::new();
    for _ in 0..NUM_LINES {
        let x = random_range(-25.0, 25.0);
        let y = random_range(-25.0, 25.0);
        lines.push(Line::new(x, y, colors[random_range(0, 4)]));
        //lines.push(Line::new(0.0, 0.0, colors[random_range(0, 4)]));
    }
    Model {
        points,
        perlin,
        lines,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //model.update_grid_noise();
    for point in &mut model.points {
        for i in 0..model.lines.len() {
            let line_point = model.lines[i].points[model.lines[i].points.len() - 1];
            if point.location.distance(line_point) < DIST {
                let noise = model.perlin.get([
                    point.noise_xt + i as f64 * 0.025,
                    point.noise_yt + i as f64 * 0.025,
                ]);
                let angle = map_range(noise, -1.0, 1.0, -2.0 * PI, 2.0 * PI);
                let x = line_point.x + angle.cos() * 0.25;
                let y = line_point.y + angle.sin() * 0.25;
                model.lines[i].points.push(pt2(x, y));
                model.lines[i].update();
            }
        }
    }
    //model.update_lines();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BEIGE);
    }
    //model.draw_grid(&draw);
    model.draw_lines(&draw);
    draw.to_frame(app, &frame).unwrap();
}
