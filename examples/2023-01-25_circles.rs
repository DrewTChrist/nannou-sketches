use nannou::prelude::*;
use nannou::color::{Alpha, Gradient, LinSrgba};
use nannou::noise::{NoiseFn, Perlin};

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    location: Vec2,
    t: f32,
    color_index: f32,
    radius: f32,
    rot_radius: f32,
}

impl Circle {
    fn new(x: f32, y: f32, t: f32, radius: f32, rot_radius: f32) -> Self {
        Self {
            location: pt2(x, y),
            t,
            //t: random_range(-500.0, 500.0),
            color_index: 0.0,
            radius,
            rot_radius
        }
    }
    fn update(&mut self) {
        self.t += 0.01;
        self.location.x = self.t.cos() * self.rot_radius;
        self.location.y = self.t.sin() * self.rot_radius;
        let perlin = Perlin::new();
        let noise = perlin.get([self.t as f64*0.1, 0.0]);
        //self.color_index = map_range(self.t.sin(), -1.0, 1.0, 0.0, 1.0);
        self.color_index = map_range(noise, -1.0, 1.0, 0.0, 1.0);
    }
    fn draw(&self, draw: &Draw, gradient: &Gradient<LinSrgba>) {
        draw.ellipse()
            .x_y(self.location.x, self.location.y)
            .radius(self.radius)
            .color(gradient.get(self.color_index));
    }
}

struct Model {
    circles: Vec<Circle>,
    gradient: Gradient<LinSrgba>,
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(600, 600)
        .view(view)
        .build()
        .unwrap();
    let mut circles = Vec::new();
    for i in 0..12 {
        circles.push(Circle::new(0.0, 0.0, i as f32, 115.0, 100.0));
    }
    let colors = vec![
        //0xffd4b2, 0xfff6bd, 0xceedc7, 0x86c8bc,
        //0x65647c, 0x8b7e74, 0xc7bca1, 0xf1d3b3
        0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62,
    ];
    let lin_colors = colors.iter().map(|c| {
        let r = c >> 16;
        let g = c >> 8 & 0x00ff;
        let b = c & 0x0000ff;
        LinSrgba::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 0.5)
    });
    let gradient = Gradient::new(lin_colors);
    Model { circles, gradient }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for circle in &mut model.circles {
        circle.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let t = app.time;
    let draw = app.draw();
    //if app.elapsed_frames() == 1 {
    //    draw.background().color(BLACK);
    //}
    //draw.rect()
    //    .x_y(0.0, 0.0)
    //    .w_h(bounds.w(), bounds.h())
    //    .color(srgba(0.0, 0.0, 0.0, 001));
    for circle in &model.circles {
        circle.draw(&draw, &model.gradient);
    }
    draw.to_frame(app, &frame).unwrap();
}
