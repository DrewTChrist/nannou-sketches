use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use palette::blend::{Equations, Parameter};
use palette::{Blend, LinSrgba};

fn main() {
    nannou::app(model).update(update).run();
}

struct Ellipse {
    x_y: Vec2,
    width: f32,
    height: f32,
    //color: Rgb<u8>,
    color: Alpha<Rgb<f32>, f32>,
    nstep: f32,
}

impl Ellipse {
    fn new(x_y: Vec2, width: f32, height: f32, color: Alpha<Rgb<f32>, f32>, nstep: f32) -> Self {
        Self {
            x_y,
            width,
            height,
            color,
            nstep,
        }
    }

    fn draw(&self, draw: &Draw, model: &Model) {
        let blend_mode =
            Equations::from_parameters(Parameter::SourceColor, Parameter::DestinationAlpha);
        let a = LinSrgba::new(
            self.color.color.red,
            self.color.color.green,
            self.color.color.blue,
            self.color.alpha,
        );
        let b = LinSrgba::new(
            model.fade.color.red,
            model.fade.color.green,
            model.fade.color.blue,
            model.fade.alpha,
        );
        let c = a.blend(b, blend_mode);
        draw.ellipse()
            //.color(self.color)
            .color(c)
            .w_h(self.width, self.height)
            .x_y(self.x_y.x, self.x_y.y);
    }
}

struct Model {
    ell: Vec<Ellipse>,
    noise: Perlin,
    fade: Alpha<Rgb<f32>, f32>,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();

    let bounds = app.window_rect();

    let mut ell = Vec::<Ellipse>::new();
    let mut x = bounds.left();
    let mut gray = 0.1;
    let mut nstep = random_range(0.0, 500.0);
    //for i in 0..50 {
    while x <= bounds.right() {
        ell.push(Ellipse::new(
            pt2(x, 0.0),
            25.0,
            250.0,
            srgba(gray, gray, gray, 0.1),
            nstep,
        ));
        //x += 30.0;
        x += 20.0;
        //gray += 0.01;
        gray += 0.05;
        //nstep += 0.125;
        nstep += 0.075;
    }

    Model {
        ell,
        noise: Perlin::new(),
        //fade: srgba(0.0, 0.0, 0.0, 0.3)
        fade: srgba(0.09, 0.09, 0.09, 0.15),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    for ell in model.ell.iter_mut() {
        let noise = map_range(
            model.noise.get([ell.nstep as f64, 0.0]),
            -1.0,
            1.0,
            //0.0,
            //bounds.top(),
            -2.0,
            2.0,
        );
        let color_noise = map_range(
            model.noise.get([ell.nstep as f64 + 50.0, 0.0]),
            -1.0,
            1.0,
            0.0,
            1.0,
        );
        //ell.width = noise;
        //ell.height = noise * 2.0;
        ell.width += noise;
        ell.height += noise * 2.0;
        ell.nstep += 0.0125;
        ell.color.red = color_noise;
        ell.color.blue = color_noise * 2.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect().w_h(600.0, 600.0).color(model.fade);

    for ell in &model.ell {
        ell.draw(&draw, &model);
    }

    draw.to_frame(app, &frame).unwrap();
}
