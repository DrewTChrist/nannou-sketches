use nannou::color::gradient::Gradient;
use nannou::geom::Rect;
use nannou::prelude::*;
use nannou::text::{font::from_file, Font};
use palette::blend::{Equations, Parameter};
use palette::{Blend, LinSrgba};

fn main() {
    nannou::app(model).update(update).run();
}

#[rustfmt::skip]
const CODE: &str = 
"
 1 struct Depth;
 2
 3 struct Adjacency;
 4
 5 fn main() {
 6    let counter = 0xdead;
 7    println!(\"{}\", counter);
 8    println!(\"Imposter\");
 9 }
10
11 struct Color;
";

struct Circle {
    radius: f32,
    color: LinSrgba,
}

struct Background {
    circles: Vec<Circle>,
    /*blend_mode: Equations,
    gradient: Gradient<LinSrgba>,*/
}

impl Background {
    fn new(num_circles: u32, gradient_step: f32, radius_step: f32) -> Self {
        let blend_mode =
            Equations::from_parameters(Parameter::SourceAlpha, Parameter::DestinationAlpha);
        let gradient = Gradient::new([
            LinSrgba::new(0.0, 0.0, 0.0, 0.05),
            LinSrgba::new(50.0, 50.0, 50.0, 0.05),
            LinSrgba::new(100.0, 100.0, 100.0, 0.05),
            LinSrgba::new(150.0, 150.0, 150.0, 0.05),
            LinSrgba::new(255.0, 255.0, 255.0, 0.05),
        ]);
        //let gradient_step = 0.0001;
        let mut circles = Vec::<Circle>::new();
        let mut gradient_count = 0.0;
        let mut radius = 10.0;
        for _i in 0..num_circles {
            let current = gradient.get(gradient_count);
            let next = gradient.get(gradient_count + gradient_step);
            let a = LinSrgba::new(current.red, current.green, current.blue, current.alpha);
            let b = LinSrgba::new(next.red, next.green, next.blue, next.alpha);
            let c = a.blend(b, blend_mode);
            circles.push(Circle { radius, color: c });
            gradient_count += gradient_step;
            radius += radius_step;
        }
        Self {
            circles,
            /*blend_mode,
            gradient,*/
        }
    }
    fn draw(&self, draw: &Draw) {
        for circle in &self.circles {
            draw.ellipse()
                .radius(circle.radius)
                .x_y(0.0, 0.0)
                .color(circle.color);
        }
    }
}

struct Model {
    font: Font,
    background: Background,
    index: usize,
    string: String,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let font = from_file("font.ttf").unwrap();
    let args: Vec<String> = std::env::args().collect();
    let circles = args[1].parse::<u32>().unwrap();
    let grad_step = args[2].parse::<f32>().unwrap();
    let rad_step = args[3].parse::<f32>().unwrap();
    Model {
        font,
        //background: Background::new(20, 0.00005, 20.0),
        background: Background::new(circles, grad_step, rad_step),
        index: 0,
        //string: format!("{}", CODE),
        string: String::from(CODE),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut tries = 0;
    let mut rand = random_range(0, 3);
    //println!("{} {} {}", model.string.len(), model.index, rand);
    while !(model.index + rand < model.string.len()) && tries < 3 {
        rand = random_range(0, 3);
        tries += 1;
    }
    if model.index + rand < model.string.len() {
        model.index += rand;
    } else if model.index + 1 < model.string.len() {
        model.index += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.rect().x_y(0.0, 0.0).w_h(300.0, 300.0).color(GRAY);
    draw.ellipse().x_y(0.0, 0.0).radius(5.0).color(RED);
    //let r = Rect::from_w_h(600.0, 600.0);
    let r = Rect::from_x_y_w_h(0.0, 0.0, 300.0, 300.0);
    let string = String::from_utf8(model.string.as_bytes()[0..model.index].to_vec()).unwrap();
    let text = text(&string)
        .font_size(12)
        .left_justify()
        .align_bottom()
        .font(model.font.clone())
        .build(r);
    draw.path().fill().color(BLACK).events(text.path_events());
    //model.background.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
