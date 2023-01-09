use nannou::color::rgb_u32;
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
    radius: f32,
    scale: f32,
    color: Rgb<u8>,
    noise: Perlin,
}

impl Hypotrochoid {
    fn new(origin: Vec2, a: f32, b: f32, h: f32, scale: f32, radius: f32, color: Rgb<u8>) -> Self {
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
            radius,
            scale,
            color,
            noise: Perlin::new(),
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
        self.t += 0.05;
        //self.t += 0.025;
        //self.x_noise += 0.003;
        //self.y_noise += 0.003;
        self.x_noise += 0.03;
        self.y_noise += 0.03;
    }
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x + self.origin.x, self.y + self.origin.y)
            .radius(self.radius)
            .color(self.color);
    }
}

struct Model {
    particles: Vec<Hypotrochoid>,
}

const NUM_PARTICLES: usize = 150;

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut particles = Vec::<Hypotrochoid>::new();
    for i in 0..NUM_PARTICLES {
        let rx = random_range(-25.0, 25.0);
        let ry = random_range(-25.0, 25.0);
        //let mut p = Hypotrochoid::new(pt2(rx, ry), 5.0, 7.0, 2.2, 60.0, 1.0, rgb_u32(0xD7E9B9));
        let mut p = Hypotrochoid::new(pt2(rx, ry), 4.0, 7.0, 2.2, 50.0, 1.5, rgb_u32(0x000000));
        //p.t = i as f32 * 0.0125;
        p.t = i as f32 * 0.05;
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
        draw.background().color(WHITE);
    }

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(app.window_rect().w(), app.window_rect().h())
        .color(srgba(1.0, 1.0, 1.0, 0.25));

    for p1 in &model.particles {
        p1.draw(&draw);
        for p2 in &model.particles {
            if (p1.x - p2.x).abs() < 30.0 && (p1.y - p2.y).abs() < 30.0 {
                draw.line()
                    .start(pt2(p1.x, p1.y))
                    .end(pt2(p2.x, p2.y))
                    .weight(1.0)
                    .color(BLACK);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
