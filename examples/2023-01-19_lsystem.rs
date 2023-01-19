//use nannou::noise::NoiseFn;
use nannou::prelude::*;
use nannou_sketches::lsystem::{LSystem, Rule};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    lsys: LSystem,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut lsys = LSystem::new("A", 8, pt2(0.0, 0.0), 0.0, 25.0, 25.0)
        .add_alphabet(vec!['A', 'B', '-', '+', '[', ']', '*'])
        .add_rule(Rule('A', String::from("[-B]"), |draw, turtle, _index| {
            //let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
            //let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
            //let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
            //let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
            turtle.forward();
            //turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
            draw.line()
                .start(turtle.previous)
                .end(turtle.position)
                .color(BEIGE)
                .weight(1.0);
        }))
        .add_rule(Rule('B', String::from("+A-B"), |draw, turtle, _index| {
            //let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
            //let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
            //let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
            //let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
            turtle.forward();
            //turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
            draw.line()
                .start(turtle.previous)
                .end(turtle.position)
                .color(BEIGE)
                .weight(1.0);
        }))
        .add_rule(Rule('+', String::from("+"), |_, turtle, _| {
            turtle.increase_angle();
        }))
        .add_rule(Rule('-', String::from("-"), |_, turtle, _| {
            turtle.decrease_angle();
        }))
        .add_rule(Rule('[', String::from("["), |_, turtle, _| {
            turtle.push_state();
        }))
        .add_rule(Rule(']', String::from("]"), |_, turtle, _| {
            turtle.pop_state();
        }))
        .add_rule(Rule('*', String::from("*"), |_, turtle, _| {
            turtle.forward();
        }));
    lsys.build();
    Model { lsys }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for noise in &mut model.lsys.noise {
        noise.0 += 0.025;
        noise.1 += 0.025;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _bounds = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);
    model.lsys.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
