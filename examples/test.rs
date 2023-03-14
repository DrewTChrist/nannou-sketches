use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Triangle {
    center: Vec2,
    big_r: f32,
}

impl Triangle {
    fn draw(&self, draw: &Draw) {}
    fn draw_corners(&self, draw: &Draw) {}
}

struct Model {
    rotation: f32,
    rect: Rect,
    center: Vec2,
    big_r: f32,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let big_r = 50.0;
    let rect = Rect::from_x_y_w_h(0.0, 0.0, 250.0, 250.0);
    let center = pt2(rect.right() - big_r, 0.0);
    Model {
        rotation: 0.0,
        rect,
        center,
        big_r,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rotation += 0.01;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let a = 3.0.sqrt() * model.big_r;
    let h = (3.0.sqrt() / 2.0) * a;
    let r = h / 3.0;
    let points: [Vec2; 3] = [
        pt2(
            model.center.x + (model.rotation + 0.0).cos() * model.big_r,
            model.center.y + (model.rotation + 0.0).sin() * model.big_r,
        ),
        pt2(
            model.center.x + (model.rotation + (2.0 * PI) / 3.0).cos() * model.big_r,
            model.center.y + (model.rotation + (2.0 * PI) / 3.0).sin() * model.big_r,
        ),
        pt2(
            model.center.x + (model.rotation + ((2.0 * PI) / 3.0) * 2.0).cos() * model.big_r,
            model.center.y + (model.rotation + ((2.0 * PI) / 3.0) * 2.0).sin() * model.big_r,
        ),
    ];
    //println!("{} {} {}", big_r, a, h);
    draw.rect()
        .x_y(model.rect.x(), model.rect.y())
        .w_h(model.rect.w(), model.rect.h())
        .color(DARKSLATEGRAY);
    draw.ellipse().xy(model.center).radius(model.big_r).color(BLUE);
    draw.ellipse()
        .xy(model.center)
        .radius(model.big_r)
        .resolution(3.0)
        .rotate(model.rotation)
        .color(WHITE);
    draw.ellipse().xy(model.center).radius(r).color(RED);
    for point in points {
        draw.ellipse().xy(point).radius(2.5).color(RED);
    }
    draw.to_frame(app, &frame).unwrap();
}
