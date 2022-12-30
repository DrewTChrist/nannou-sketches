use nannou::color::gradient::Gradient;
use nannou::prelude::*;
use palette::blend::{Equations, Parameter};
use palette::{Blend, LinSrgba};

pub struct Circle {
    radius: f32,
    color: LinSrgba,
}

pub struct Background {
    circles: Vec<Circle>,
    /*blend_mode: Equations,
    gradient: Gradient<LinSrgba>,*/
}

impl Background {
    pub fn new(num_circles: u32, gradient_step: f32, radius_step: f32) -> Self {
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
    pub fn draw(&self, draw: &Draw) {
        for circle in &self.circles {
            draw.ellipse()
                .radius(circle.radius)
                .x_y(0.0, 0.0)
                .color(circle.color);
        }
    }
}
