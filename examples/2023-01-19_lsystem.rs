use nannou::noise::NoiseFn;
use nannou::prelude::*;
use nannou_sketches::lsystem::{default_rules, LSystem, Rule};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    lsys: LSystem,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut lsys = LSystem::new("F+F+F+F", 2, pt2(0.0, 0.0), 0.0, 90.0, 10.0)
        .add_rule(Rule(
            'F',
            String::from("FF+F+F+F+F+F-F"),
            |draw, turtle, index| {
                turtle.forward();
                if index < turtle.noise.len() {
                    let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
                    let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
                    let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
                    let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
                    turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
                }
                draw.line()
                    .start(turtle.previous)
                    .end(turtle.position)
                    .color(BEIGE)
                    .weight(1.0);
            },
        ))
        //.add_rule(Rule('X', String::from("-YF+XFX+FY-"), |_, _, _| {}))
        //.add_rule(Rule('Y', String::from("+XF-YFY-FX+"), |_, _, _| {}))
        .add_rule(default_rules::left_bracket())
        .add_rule(default_rules::right_bracket())
        .add_rule(default_rules::plus())
        .add_rule(default_rules::minus());
    lsys.build();
    Model { lsys }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for noise in &mut model.lsys.noise {
        noise.0 += 0.005;
        noise.1 += 0.005;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _bounds = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);
    model.lsys.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
