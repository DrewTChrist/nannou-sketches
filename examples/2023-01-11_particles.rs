use nannou::color::rgb_u32;
use nannou::geom::Rect;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Hypotrochoid {
    origin: Vec2,
    x: f32,
    y: f32,
    x_noise: f64,
    y_noise: f64,
    a: f32,
    b: f32,
    h: f32,
    t: f32,
    scale: f32,
    color: Rgb<u8>,
    noise: Perlin,
    string: String,
}

impl Hypotrochoid {
    fn new(origin: Vec2, a: f32, b: f32, h: f32, scale: f32, color: Rgb<u8>) -> Self {
        Self {
            origin,
            x: 0.0,
            y: 0.0,
            x_noise: random_range(-500.0, 500.0),
            y_noise: random_range(-500.0, 500.0),
            a,
            b,
            h,
            t: random_range(-500.0, 500.0),
            scale,
            color,
            noise: Perlin::new(),
            string: String::new(),
        }
    }
    fn increment(&mut self) {
        let xn = map_range(self.noise.get([self.x_noise, 0.0]), -1.0, 1.0, -50.0, 50.0);
        let yn = map_range(self.noise.get([self.y_noise, 0.0]), -1.0, 1.0, -50.0, 50.0);
        self.x = (self.a - self.b) * self.t.cos()
            + self.h * (((self.a - self.b) / self.b) * self.t).cos();
        self.y = (self.a - self.b) * self.t.sin()
            + self.h * (((self.a - self.b) / self.b) * self.t).sin();
        self.x *= self.scale;
        self.y *= self.scale;
        self.x += xn;
        self.y += yn;
        self.t += 0.01;
        self.x_noise += 0.003;
        self.y_noise += 0.003;
    }
    fn draw(&self, draw: &Draw) {
        let start = pt2(self.x + self.origin.x, self.y + self.origin.y);
        let rect = Rect::from_x_y_w_h(start.x + 10.0, start.y + 10.0, 40.0, 15.0);
        let text = text(&self.string)
            .font_size(16)
            .left_justify()
            .align_bottom()
            //.font(self.font.clone())
            .build(rect);
        draw.path()
            .fill()
            .color(self.color)
            .events(text.path_events());
        draw.ellipse()
            .x_y(start.x, start.y)
            .radius(2.5)
            .color(self.color);
    }
}

struct Model {
    particles: Vec<Hypotrochoid>,
}

//const NUM_PARTICLES: usize = 10;

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut particles = Vec::<Hypotrochoid>::new();
    let args: Vec<String> = std::env::args().collect();
    let split: Vec<&str> = args[1].split(' ').collect();
    //for i in 0..NUM_PARTICLES {
    for i in 0..split.len() {
        let rx = random_range(-25.0, 25.0);
        let ry = random_range(-25.0, 25.0);
        let mut p = Hypotrochoid::new(pt2(rx, ry), 4.0, 7.0, 2.2, 50.0, rgb_u32(0x000000));
        p.string.push_str(split[i]);
        p.t = i as f32 * 0.025;
        particles.push(p);
    }
    Model { particles }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for particle in model.particles.iter_mut() {
        particle.increment();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BEIGE);
    }

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(app.window_rect().w(), app.window_rect().h())
        .color(srgba(
            BEIGE.red as f32 / 255.0,
            BEIGE.green as f32 / 255.0,
            BEIGE.blue as f32 / 255.0,
            0.25,
        ));

    for p in &model.particles {
        p.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
