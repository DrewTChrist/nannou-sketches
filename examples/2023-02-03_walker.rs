use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::{
    gradient::{grad_many, gradient},
    utilities::u32_to_srgba,
};

fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    location: Vec2,
    color: Alpha<Rgb<f32>, f32>,
}

impl Point {
    fn new(x: f32, y: f32, color: Alpha<Rgb<f32>, f32>) -> Self {
        Self {
            location: pt2(x, y),
            color,
        }
    }
}

struct Walker {
    points: Vec<Point>,
    perlin: Perlin,
    t: f64,
    color_t: f32,
    colors: Vec<Alpha<Rgb<f32>, f32>>,
}

impl Walker {
    fn new(x: f32, y: f32, colors: Vec<Alpha<Rgb<f32>, f32>>) -> Self {
        let mut points = Vec::new();
        points.push(Point::new(x, y, srgba(1.0, 1.0, 1.0, 1.0)));
        Self {
            points,
            perlin: Perlin::new(),
            t: random_range(-500.0, 500.0),
            color_t: random_range(-500.0, 500.0),
            colors,
        }
    }
    fn update(&mut self) {
        let prev = &self.points[self.points.len() - 1];
        let noise = self.perlin.get([self.t, 0.0]);
        let color_noise = self.perlin.get([self.color_t as f64, 0.0]);
        let color_nmap = map_range(color_noise, -1.0, 1.0, 0.0, 1.0);
        let angle = map_range(noise, -1.0, 1.0, -2.0 * PI, 2.0 * PI);
        let x = angle.cos() * 0.5;
        let y = angle.sin() * 0.5;
        let point = Point::new(
            prev.location.x + x,
            prev.location.y + y,
            grad_many(&self.colors, color_nmap),
        );
        self.points.push(point);
        //self.t += 0.00625;
        //self.t += 0.01;
        //self.t += 0.0005;
        self.t += 0.00001;
        //self.color_t += 0.03;
        self.color_t += 0.05;
    }
    fn remove_point(&mut self) {
        self.points.remove(0);
    }
    fn draw(&self, draw: &Draw) {
        let mut step = 0.0;
        for i in 1..self.points.len() {
            let noise = self.perlin.get([self.t + step, 0.0]);
            let noise_map = map_range(noise, -1.0, 1.0, 0.0, 250.0);
            draw.line()
                .start(self.points[i - 1].location)
                .end(self.points[i].location)
                .rotate(map_range(noise, -1.0, 1.0, -2.0 * PI, 2.0 * PI))
                .weight(noise_map)
                .color(self.points[i].color);
            //step += 0.0005;
            step += 2.0;
        }
    }
}

struct Model {
    walkers: Vec<Walker>,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let alpha = 0.5;
    let colors = vec![
        u32_to_srgba(0xa7d2cb, alpha),
        u32_to_srgba(0xf2d388, alpha),
        u32_to_srgba(0xc98474, alpha),
        u32_to_srgba(0x874c62, alpha),
    ];
    let mut walkers = Vec::new();
    for _ in 0..25 {
        walkers.push(Walker::new(0.0, 0.0, colors.clone()));
    }
    Model { walkers }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for walker in &mut model.walkers {
        walker.update();
        if walker.points.len() > 5 {
            walker.remove_point();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(DARKSLATEGRAY);
    }
    //draw.rect()
    //    .x_y(0.0, 0.0)
    //    .w_h(bounds.w(), bounds.h())
    //    .color(srgba(0.0, 0.0, 0.0, 0.1));
    for walker in &model.walkers {
        walker.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
