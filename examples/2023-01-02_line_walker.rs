use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Vec2>,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut points = Vec::<Vec2>::new();
    points.push(pt2(0.0, 0.0));
    Model { points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    let last_point = model.points[0];
    let rand_point = pt2(
        random_range(last_point.x - 300.0, last_point.x + 300.0),
        random_range(last_point.y - 300.0, last_point.y + 300.0),
    );
    if rand_point.x > bounds.left()
        && rand_point.x < bounds.right()
        && rand_point.y > bounds.bottom()
        && rand_point.y < bounds.top()
    {
        model.points.push(rand_point);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.polyline()
        .points(model.points.clone())
        //.weight(5.0)
        .color(WHITE);
    for point in model.points.iter() {
        draw.ellipse()
            .x_y(point.x, point.y)
            .w_h(5.0, 5.0)
            .color(GRAY);
    }
    draw.to_frame(app, &frame).unwrap();
}
