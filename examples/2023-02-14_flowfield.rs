use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::utilities::u32_to_srgba;

fn main() {
    nannou::app(model).update(update).run();
}

struct FlowField {
    grid: Vec<Point>,
    lines: Vec<Line>,
    perlin: Perlin,
    distance: f32,
    length: f32,
}

impl FlowField {
    fn new(
        width: isize,
        height: isize,
        resolution: usize,
        num_lines: usize,
        distance: f32,
        length: f32,
        noise_t: f64,
        noise_scale: f64,
        coordinate_scale: f64,
    ) -> Self {
        let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62]
            .iter()
            .map(|c| u32_to_srgba(*c, 0.7))
            .collect();
        let mut lines = Vec::new();
        let mut grid = Vec::new();
        let mut noise_xt = random_range(-5000.0, 5000.0);
        let mut noise_yt = random_range(-5000.0, 5000.0);
        //let mut noise_xt = 1.0;
        //let mut noise_yt = 0.0;
        let perlin = Perlin::new();
        for i in (-width..width).step_by(resolution) {
            for j in (-height..height).step_by(resolution) {
                let noise = perlin.get([
                    (noise_xt * noise_scale) + i as f64 * coordinate_scale,
                    (noise_yt * noise_scale) + j as f64 * coordinate_scale,
                ]);
                let angle = map_range(noise, -1.0, 1.0, -2.0 * PI, 2.0 * PI);
                grid.push(Point::new(i as f32, j as f32, angle));
                noise_xt += noise_t;
                noise_yt += noise_t;
            }
        }
        let mut line_points = Vec::new();
        for _ in 0..num_lines {
            let x = random_range(-width as f32, width as f32);
            let y = random_range(-height as f32, height as f32);
            //let x = random_range(-100.0, 100.0);
            //let y = random_range(-100.0, 100.0);
            if !line_points.contains(&pt2(x, y)) {
                line_points.push(pt2(x, y));
                lines.push(Line::new(x, y, colors[random_range(0, 4)]));
            }
        }
        Self {
            grid,
            lines,
            perlin,
            distance,
            length,
        }
    }
    fn update(&mut self) {
        for point in &mut self.grid {
            for i in 0..self.lines.len() {
                let line_point = self.lines[i].points[self.lines[i].points.len() - 1];
                if point.location.distance(line_point) <= self.distance {
                    //let r = random_range(0.0, 10.0);
                    //let chance = if r >= 5.0 { 32.0 } else { 0.0 };
                    //let x = line_point.x + (point.angle + chance).cos() * 1.0;
                    //let y = line_point.y + (point.angle + chance).sin() * 1.0;
                    let x = line_point.x + point.angle.cos() * self.length;
                    let y = line_point.y + point.angle.sin() * self.length;
                    self.lines[i].push(pt2(x, y));
                    self.lines[i].update();
                }
            }
        }
        //for line in &mut self.lines {
        //    let line_point = line.points[line.points.len() - 1];
        //    let mut nearest = None;
        //    let mut dist = 1000000000.0;
        //    for i in 0..self.grid.len() {
        //        let cur_dist = line_point.distance(self.grid[i].location);
        //        if cur_dist <= dist {
        //            nearest = Some(&self.grid[i]);
        //            dist = cur_dist;
        //        }
        //    }
        //    if let Some(n) = nearest {
        //        let x = line_point.x + n.angle.cos() * self.length;
        //        let y = line_point.y + n.angle.sin() * self.length;
        //        line.push(pt2(x, y));
        //        line.update();
        //    }
        //}
    }
    fn draw_grid(&self, draw: &Draw) {
        for point in &self.grid {
            let x = point.location.x + point.angle.cos() * 7.5;
            let y = point.location.y + point.angle.sin() * 7.5;
            draw.line()
                .start(point.location)
                .end(pt2(x, y))
                .weight(2.0)
                .color(RED);
        }
    }
    fn draw_lines(&self, draw: &Draw) {
        for line in &self.lines {
            line.draw(&draw);
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

struct Line {
    points: Vec<Vec2>,
    color: Alpha<Rgb<f32>, f32>,
    len: usize,
}

impl Line {
    fn new(x: f32, y: f32, color: Alpha<Rgb<f32>, f32>) -> Self {
        let mut points = Vec::new();
        points.push(pt2(x, y));
        Self { points, color, len: 1 }
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

struct Model {
    field: FlowField,
    grid_on: bool,
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(600, 600)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .view(view)
        .build()
        .unwrap();
    //let field = FlowField::new(300, 300, 3, 1200, 5.0, 5.0, 0.003, 0.001, 0.005);
    //let field = FlowField::new(300, 300, 3, 1000, 5.0, 2.0, 0.003, 0.001, 0.0025);
    //let field = FlowField::new(350, 350, 50, 5000, 5.0, 0.5, 0.001, 0.001, 0.001); //this setting was really nice
    let field = FlowField::new(350, 350, 10, 3000, 15.0, 2.0, 0.005, 0.005, 0.001); //this setting was really nice
    //let field = FlowField::new(350, 350, 10, 3000, 5.0, 2.5, 0.002, 0.001, 0.003);
    Model {
        field,
        grid_on: false,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //if _app.elapsed_frames() < 5 {
    model.field.update();
    //}
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(DARKSLATEGRAY);
    }
    model.field.draw_lines(&draw);
    if model.grid_on {
        model.field.draw_grid(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if let Key::A = key {
        model.grid_on = true;
    }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    if let Key::A = key {
        model.grid_on = false;
    }
}
