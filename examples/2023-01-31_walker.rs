use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};
//use std::cmp::{min,max};

fn main() {
    nannou::app(model).update(update).run();
}

fn on_segment(p: Vec2, q: Vec2, r: Vec2) -> bool {
    //if (q.x <= max(p.x, r.x) && q.x >= min(p.x, r.x) &&
    //    q.y <= max(p.y, r.y) && q.y >= min(p.y, r.y)) {
    if (q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
        q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)) {
       return true;
    }

    return false;
}

// To find orientation of ordered triplet (p, q, r).
// The function returns following values
// 0 --> p, q and r are collinear
// 1 --> Clockwise
// 2 --> Counterclockwise
fn orientation(p: Vec2, q: Vec2, r: Vec2) -> u32 {
    // See https://www.geeksforgeeks.org/orientation-3-ordered-points/
    // for details of below formula.
    let val = (q.y - p.y) * (r.x - q.x) -
              (q.x - p.x) * (r.y - q.y);

    if val == 0.0 {
        return 0;  
    }// collinear

    return if val > 0.0 { 1 } else { 2 }; // clock or counterclock wise
}

// The main function that returns true if line segment 'p1q1'
// and 'p2q2' intersect.
fn intersect(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> bool {
    // Find the four orientations needed for general and
    // special cases
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases
    // p1, q1 and p2 are collinear and p2 lies on segment p1q1
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }

    // p1, q1 and q2 are collinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }

    // p2, q2 and p1 are collinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }

     // p2, q2 and q1 are collinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    return false; // Doesn't fall in any of the above cases
}

struct Walker {
    points: Vec<Vec2>,
    perlin: Perlin,
    t: f64
}

impl Walker {
    fn new(x: f32, y: f32) -> Self {
        let mut points = Vec::new();
        points.push(pt2(x, y));
        Self {
            points,
            perlin: Perlin::new(),
            t: random_range(-500.0, 500.0)
        }
    }
    fn update(&mut self) {
        let prev = self.points[self.points.len() - 1];
        let noise = self.perlin.get([self.t, 0.0]);
        let angle = map_range(noise, -1.0, 1.0, -2.0 * PI, 2.0 * PI);
        let x = angle.cos() * 1.0;
        let y = angle.sin() * 1.0;
        let point = pt2(prev.x + x, prev.y + y);
        self.points.push(point);
        self.t += 0.05;
    }
    fn draw(&self, draw: &Draw) {
        for i in 1..self.points.len() {
            draw.line()
                .start(self.points[i - 1])
                .end(self.points[i])
                .weight(1.0)
                .color(WHITE);
        }
    }
}

struct Model {
    walkers: Vec<Walker>,
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(600, 600)
        .view(view)
        .build()
        .unwrap();
    let mut walkers = Vec::new();
    walkers.push(Walker::new(0.0, 0.0));
    Model {
        walkers,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for walker in &mut model.walkers {
        walker.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for walker in &model.walkers {
        walker.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
