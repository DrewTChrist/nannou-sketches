use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

const RADIUS: f32 = 20.0;
const FLOOR: f32 = 0.0;

trait Collideable {
    fn shape(&self) -> Shape;
}

#[derive(Copy, Clone)]
enum Shape {
    Rect,
    Ellipse,
    Line,
}

#[derive(Copy, Clone)]
struct Floor {
    start: Vec2,
    end: Vec2,
    shape: Shape,
}

impl Floor {
    fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            start,
            end,
            shape: Shape::Line,
        }
    }
}

impl Collideable for Floor {
    fn shape(&self) -> Shape {
        self.shape
    }
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
}

impl Ball {
    fn new(x: f32, y: f32, vx: f32, vy: f32, radius: f32) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            radius,
        }
    }
    fn increment(&mut self, elapsed_time: f32) {
        self.vy -= 9.8 * elapsed_time;
        self.x += self.vx * elapsed_time;
        self.y += self.vy * elapsed_time;
    }
    //fn collide(&mut self, objects: Vec<impl Collideable>) {
    fn collide(&mut self, objects: Vec<Floor>) {
        for object in objects {
            match object.shape() {
                Shape::Ellipse => {}
                Shape::Line => {
                    let end = object.end;
                    let rad = end.y.atan2(end.x);
                    let deg = rad * 180.0 / PI;
                }
                Shape::Rect => {}
            }
        }
    }
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(self.radius)
            .color(WHITE);
    }
}

struct Model {
    ball: Ball,
    floor: Floor,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn model(app: &App) -> Model {
    let start = pt2(app.window_rect().left(), FLOOR);
    let end = pt2(app.window_rect().right(), FLOOR);
    Model {
        ball: Ball::new(-100.0, 175.0, 0.0, 0.0, 20.0),
        //floor: Floor::new(start, end),
        floor: Floor::new(start, pt2(100.0, 100.0)),
        x: 0.0,
        y: 175.0,
        vx: 0.0,
        vy: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    //let elapsed_time = app.duration.since_prev_update.as_secs_f32();
    let elapsed_time = app.elapsed_frames() as f32 / 60.0;

    model.vy -= 9.8 * elapsed_time;

    model.x += model.vx * elapsed_time;
    model.y += model.vy * elapsed_time;

    if model.y <= FLOOR + RADIUS {
        model.y = FLOOR + RADIUS;
        model.vy = -model.vy * 0.8;
    }
    model.ball.increment(elapsed_time);
    model.ball.collide(vec![model.floor]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.ellipse()
        .x_y(model.x, model.y)
        .radius(RADIUS)
        .color(WHITE);

    model.ball.draw(&draw);

    let start = pt2(app.window_rect().left(), FLOOR);
    let end = pt2(app.window_rect().right(), FLOOR);

    draw.line().start(start).end(end).weight(3.0).color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
