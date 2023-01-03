//! Import photo and create a circular wave ((()))
//! of moving "glitch"
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct GlitchCircle {
    points: Vec<Vec2>,
    radius: f32,
}

impl GlitchCircle {
    fn new(radius: f32) -> Self {
        let mut points = Vec::<Vec2>::new();
        let mut t = 0.0;
        while t <= 2.0 * PI {
            let x = t.cos() * radius;
            let y = t.sin() * radius;
            points.push(pt2(x, y));
            t += 0.01;
        }
        Self { points, radius }
    }
    fn draw(&self, draw: &Draw) {
        draw.polyline().points(self.points.clone()).color(WHITE);
    }
}

struct Model {
    glitch: GlitchCircle,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("test_img.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Model {
        glitch: GlitchCircle::new(100.0),
        texture,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.texture(&model.texture);
    model.glitch.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
