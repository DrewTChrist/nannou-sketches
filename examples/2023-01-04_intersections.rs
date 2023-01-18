use nannou::color::Alpha;
use nannou::noise::{Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn u32_to_srgba(color: u32, alpha: f32) -> Alpha<Rgb<f32>, f32> {
    let r = color >> 16;
    let g = color >> 8 & 0x00ff;
    let b = color & 0x0000ff;
    srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, alpha)
}

struct Palette {
    colors: [Alpha<Rgb<f32>, f32>; 4],
}

impl Palette {
    fn new(
        _1: Alpha<Rgb<f32>, f32>,
        _2: Alpha<Rgb<f32>, f32>,
        _3: Alpha<Rgb<f32>, f32>,
        _4: Alpha<Rgb<f32>, f32>,
    ) -> Self {
        Self {
            colors: [_1, _2, _3, _4],
        }
    }
}

enum SecondStripe {
    Single,
    Double,
    NoSecond,
}

struct Plaid {
    stripes: usize,
    spread: f32,
    primary_weight: f32,
    secondary_weight: f32,
    colors: Palette,
    secondary: SecondStripe,
}

impl Plaid {
    fn new(
        stripes: usize,
        spread: f32,
        primary_weight: f32,
        secondary_weight: f32,
        secondary: SecondStripe,
        colors: Palette,
    ) -> Self {
        Self {
            stripes,
            spread,
            secondary,
            primary_weight,
            secondary_weight,
            colors,
        }
    }
    fn secondary_stripe_vertical(&self, draw: &Draw, bounds: Rect) {
        match self.secondary {
            SecondStripe::Single => {
                let mut x = 0.0 - ((self.stripes as f32 - 1.0) * self.spread) / 2.0;
                for _ in 0..self.stripes {
                    let start = pt2(x, bounds.bottom());
                    let end = pt2(x, bounds.top());
                    draw.line()
                        .start(start)
                        .end(end)
                        .weight(self.secondary_weight)
                        .color(self.colors.colors[2]);
                    x += self.spread;
                }
            }
            SecondStripe::Double => {
                let mut x = 0.0 - ((self.stripes as f32 - 1.0) * self.spread) / 2.0;
                for _ in 0..self.stripes {
                    let start = pt2(x, bounds.bottom());
                    let end = pt2(x, bounds.top());
                    draw.line()
                        .start(pt2(start.x + 5.0, start.y))
                        .end(pt2(end.x + 5.0, end.y))
                        .weight(self.secondary_weight)
                        .color(self.colors.colors[2]);
                    draw.line()
                        .start(pt2(start.x - 5.0, start.y))
                        .end(pt2(end.x - 5.0, end.y))
                        .weight(self.secondary_weight)
                        .color(self.colors.colors[2]);
                    x += self.spread;
                }
            }
            SecondStripe::NoSecond => {}
        }
    }
    fn secondary_stripe_horizontal(&self, draw: &Draw, bounds: Rect) {
        match &self.secondary {
            SecondStripe::Single => {
                let mut y = 0.0 - ((self.stripes as f32 - 1.0) * self.spread) / 2.0;
                for _ in 0..self.stripes {
                    let start = pt2(bounds.right(), y);
                    let end = pt2(bounds.left(), y);
                    draw.line()
                        .start(start)
                        .end(end)
                        .weight(self.secondary_weight)
                        .color(self.colors.colors[2]);
                    y += self.spread;
                }
            }
            SecondStripe::Double => {
                let mut y = 0.0 - ((self.stripes as f32 - 1.0) * self.spread) / 2.0;
                for _ in 0..self.stripes {
                    let start = pt2(bounds.right(), y);
                    let end = pt2(bounds.left(), y);
                    draw.line()
                        .start(pt2(start.x, start.y + 5.0))
                        .end(pt2(end.x, end.y + 5.0))
                        .weight(self.secondary_weight)
                        .color(self.colors.colors[2]);
                    draw.line()
                        .start(pt2(start.x, start.y - 5.0))
                        .end(pt2(end.x, end.y - 5.0))
                        .weight(self.secondary_weight)
                        .color(self.colors.colors[2]);
                    y += self.spread;
                }
            }
            SecondStripe::NoSecond => {}
        }
    }
    fn vertical(&self, draw: &Draw, bounds: Rect) {
        let mut x = 0.0 - ((self.stripes as f32 - 1.0) * self.spread) / 2.0;
        for _ in 0..self.stripes {
            let start = pt2(x, bounds.bottom());
            let end = pt2(x, bounds.top());
            draw.line()
                .start(start)
                .end(end)
                .weight(self.primary_weight)
                .color(self.colors.colors[0]);
            x += self.spread;
        }
    }
    fn horizontal(&self, draw: &Draw, bounds: Rect) {
        let mut y = 0.0 - ((self.stripes as f32 - 1.0) * self.spread) / 2.0;
        for _ in 0..self.stripes {
            let start = pt2(bounds.right(), y);
            let end = pt2(bounds.left(), y);
            draw.line()
                .start(start)
                .end(end)
                .weight(self.primary_weight)
                .color(self.colors.colors[1]);
            y += self.spread;
        }
    }
    fn draw(&self, draw: &Draw, bounds: Rect) {
        let _x = || {
            draw.rect()
                .x_y(0.0, 0.0)
                .w_h(bounds.w(), bounds.h())
                //.color(srgba(0.0, 0.0, 0.0, 0.5));
                .color(self.colors.colors[3]);
        };

        self.vertical(draw, bounds);
        self.secondary_stripe_vertical(draw, bounds);
        self.horizontal(draw, bounds);
        self.secondary_stripe_horizontal(draw, bounds);
    }
}

struct Model {
    plaid: Plaid,
    noise: Perlin,
    nvals: [f64; 3],
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let colors = Palette::new(
        u32_to_srgba(0xfaab78, 0.5),
        u32_to_srgba(0xffdca9, 0.5),
        u32_to_srgba(0xfcf9be, 0.5),
        u32_to_srgba(0xe8f3d6, 0.1),
    );
    Model {
        plaid: Plaid::new(3, 200.0, 150.0, 7.5, SecondStripe::Double, colors),
        noise: Perlin::new(),
        nvals: [
            random_range(-500.0, 500.0),
            random_range(-500.0, 500.0),
            random_range(-500.0, 500.0),
        ],
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //let rn = model.noise.get([model.nvals[0], 0.0]) * 0.05;
    //let gn = model.noise.get([model.nvals[1], 0.0]) * 0.05;
    //let bn = model.noise.get([model.nvals[2], 0.0]) * 0.05;
    //for color in model.plaid.colors.colors.iter_mut() {
    //    color.red += rn as f32;
    //    color.green += gn as f32;
    //    color.blue += bn as f32;
    //}
    //model.nvals[0] += 0.1;
    //model.nvals[1] += 0.1;
    //model.nvals[2] += 0.1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let draw = app.draw();
    //draw.background().color(BLACK);
    model.plaid.draw(&draw, bounds);
    draw.to_frame(app, &frame).unwrap();
}
