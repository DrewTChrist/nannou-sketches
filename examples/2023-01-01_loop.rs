use nannou::color::rgb_u32;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Hypotrochoid {
    x: f32,
    y: f32,
    a: f32,
    b: f32,
    h: f32,
    t: f32,
    color: Rgb<u8>,
}

impl Hypotrochoid {
    fn new(x: f32, y: f32, a: f32, b: f32, h: f32, color: Rgb<u8>) -> Self {
        Self {
            x,
            y,
            a,
            b,
            h,
            t: 0.0,
            color,
        }
    }
    fn draw(&self, draw: &Draw) {
        let mut x = (self.a - self.b) * self.t.cos()
            + self.h * (((self.a - self.b) / self.b) * self.t).cos();
        let mut y = (self.a - self.b) * self.t.sin()
            + self.h * (((self.a - self.b) / self.b) * self.t).sin();
        x *= 50.0;
        y *= 50.0;
        draw.ellipse()
            .x_y(x + self.x, y + self.y)
            .w_h(20.0, 20.0)
            .color(self.color);
    }
}

struct Model {
    h: Hypotrochoid,
    i: Hypotrochoid,
    j: Hypotrochoid,
    k: Hypotrochoid,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    Model {
        h: Hypotrochoid::new(0.0, 20.0, 5.0, 7.0, 2.2, rgb_u32(0xD7E9B9)),
        i: Hypotrochoid::new(20.0, 0.0, 5.0, 7.0, 2.2, rgb_u32(0xFFFBAC)),
        j: Hypotrochoid::new(-20.0, 0.0, 5.0, 7.0, 2.2, rgb_u32(0xFFD495)),
        k: Hypotrochoid::new(0.0, -20.0, 5.0, 7.0, 2.2, rgb_u32(0xFAAB78)),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.h.t += 0.05;
    model.i.t += 0.05;
    model.j.t += 0.05;
    model.k.t += 0.05;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(srgb(0.3960, 0.3921, 0.4862));
    }

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(600.0, 600.0)
        .color(srgba(0.3960, 0.3921, 0.4862, 0.1));

    model.h.draw(&draw);
    model.i.draw(&draw);
    model.j.draw(&draw);
    model.k.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
