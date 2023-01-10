use nannou::prelude::*;
use nannou_sketches::path::{Drawable, Path};

fn main() {
    nannou::app(model).update(update).run();
}

struct Circ {
    x: f32,
    y: f32,
    radius: f32,
}

impl Drawable for Circ {
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(self.radius)
            .color(WHITE);
    }
    fn update_xy(&mut self, x: f32, y: f32) {
        self.x = x * 50.0;
        self.y = y * 50.0;
    }
}

struct Model {
    path: Path<Circ>,
}

fn x_func(p: &mut Path<Circ>) {
    p.x = (p.p["a"] - p.p["b"]) * p.t.cos()
        + p.p["h"] * (((p.p["a"] - p.p["b"]) / p.p["b"]) * p.t).cos();
}

fn y_func(p: &mut Path<Circ>) {
    p.y = (p.p["a"] - p.p["b"]) * p.t.sin()
        + p.p["h"] * (((p.p["a"] - p.p["b"]) / p.p["b"]) * p.t).sin();
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut path = Path::new(pt2(0.0, 0.0), 10.0, x_func, y_func);
    path.p.insert("a".into(), 4.0);
    path.p.insert("b".into(), 7.0);
    path.p.insert("h".into(), 2.2);
    path.objects.push(Circ {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    });
    path.objects.push(Circ {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    });
    Model { path }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.path.increment();
    model.path.update_x();
    model.path.update_y();
    model.path.update_objects();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.path.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
