use nannou::color::rgb_u32;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_sketches::capturer::FrameCapturer;

fn main() {
    nannou::app(model).update(update).exit(exit).run();
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
    rwing_t: f32,
    lwing_t: f32,
    wing_speed: f32,
    scale: f32,
    color: Rgb<u8>,
    noise: Perlin,
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
            rwing_t: 0.0,
            lwing_t: PI,
            wing_speed: random_range(0.1, 0.4),
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
        self.t += 0.01;
        self.x_noise += 0.003;
        self.y_noise += 0.003;
        self.rwing_t += self.wing_speed;
        self.lwing_t += self.wing_speed;
    }
    fn draw(&self, draw: &Draw) {
        let start = pt2(self.x + self.origin.x, self.y + self.origin.y);
        let span = 5.0;
        let r_range = map_range(self.rwing_t.sin(), -1.0, 1.0, 0.0 + 0.25, (PI / 2.0) - 0.25);
        let l_range = map_range(self.lwing_t.sin(), -1.0, 1.0, (PI / 2.0) + 0.25, PI - 0.25);
        let rwing_end = pt2(
            start.x + r_range.cos() * span,
            start.y + r_range.sin() * span,
        );
        let lwing_end = pt2(
            start.x + l_range.cos() * span,
            start.y + l_range.sin() * span,
        );
        draw.line().start(start).end(rwing_end).color(self.color);
        draw.line().start(start).end(lwing_end).color(self.color);
    }
}

struct Model {
    particles: Vec<Hypotrochoid>,
    capturer: FrameCapturer,
}

const NUM_PARTICLES: usize = 100;
//const TEXTURE_SIZE: [u32; 2] = [3_840, 2_160];
const TEXTURE_SIZE: [u32; 2] = [600, 600];

fn model(app: &App) -> Model {
    //let [win_w, win_h] = [TEXTURE_SIZE[0] / 4, TEXTURE_SIZE[1] / 4];
    let [win_w, win_h] = [TEXTURE_SIZE[0], TEXTURE_SIZE[1]];
    let window_id = app
        .new_window()
        .size(win_w, win_h)
        .view(view)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    std::fs::create_dir_all(&capture_directory(app)).unwrap();

    let mut particles = Vec::<Hypotrochoid>::new();
    for i in 0..NUM_PARTICLES {
        let rx = random_range(-25.0, 25.0);
        let ry = random_range(-25.0, 25.0);
        let mut p = Hypotrochoid::new(pt2(rx, ry), 4.0, 7.0, 2.2, 50.0, rgb_u32(0x000000));
        p.t = i as f32 * 0.025;
        particles.push(p);
    }
    Model {
        particles,
        capturer: FrameCapturer::new(&window, TEXTURE_SIZE),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let draw = &model.capturer.draw;
    draw.reset();

    for particle in model.particles.iter_mut() {
        particle.increment();
    }

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

    model.capturer.capture(&app);
}

fn view(_app: &App, model: &Model, frame: Frame) {
    let mut encoder = frame.command_encoder();
    model
        .capturer
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

fn exit(app: &App, model: Model) {
    let window = app.main_window();
    let device = window.device();
    model
        .capturer
        .texture_capturer
        .await_active_snapshots(&device)
        .unwrap();
}

fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}
