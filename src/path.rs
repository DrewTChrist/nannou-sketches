use nannou::prelude::*;
use std::collections::HashMap;

pub trait Drawable {
    fn draw(&self, draw: &Draw);
    fn update_xy(&mut self, x: f32, y: f32);
}

pub struct Path<D>
where
    D: Drawable,
{
    pub p: HashMap<String, f32>,
    pub objects: Vec<D>,
    pub origin: Vec2,
    pub x: f32,
    pub y: f32,
    pub t: f32,
    pub spread: f32,
    pub x_func: Option<fn(&mut Path<D>) -> ()>,
    pub y_func: Option<fn(&mut Path<D>) -> ()>,
}

impl<D> Path<D>
where
    D: Drawable,
{
    pub fn new(
        origin: Vec2,
        spread: f32,
        x_func: fn(&mut Path<D>) -> (),
        y_func: fn(&mut Path<D>) -> (),
    ) -> Self {
        Self {
            p: HashMap::<String, f32>::new(),
            objects: Vec::<D>::new(),
            origin,
            x: 0.0,
            y: 0.0,
            t: random_range(-500.0, 500.0),
            spread,
            x_func: Some(x_func),
            y_func: Some(y_func),
        }
    }
    pub fn increment(&mut self) {
        self.t += 0.01;
    }
    pub fn update_x(&mut self) {
        if let Some(xf) = self.x_func {
            (xf)(self);
        }
    }
    pub fn update_y(&mut self) {
        if let Some(yf) = self.y_func {
            (yf)(self);
        }
    }
    pub fn update_objects(&mut self) {
        for object in self.objects.iter_mut() {
            object.update_xy(self.x, self.y);
        }
    }
    pub fn draw(&self, draw: &Draw) {
        for object in &self.objects {
            object.draw(draw);
        }
    }
}
