use nannou::color::Blend;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use nannou_sketches::lsystem::{LSystem, Rule, Turtle};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    lsys: LSystem,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    //let mut lsys = LSystem::new("X", Turtle::new(pt2(0.0, 0.0), 90.0, 25.0, 5.0), 5)
    //    .add_rule(Rule(
    //        'X',
    //        String::from("F+[[X]-X]-F[-FX]+X"),
    //        //String::from("F+[[X]-X]-F-[FX]+X"),
    //        |_, _, _| {},
    //    ))
    //    .add_rule(Rule('F', String::from("FF"), |draw, turtle, index| {
    //        let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
    //        let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
    //        let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
    //        let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
    //        turtle.forward();
    //        turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .weight(1.0)
    //            .color(WHITE);
    //    }))
    //    .add_rule(Rule('-', String::from('-'), |_, turtle, _| {
    //        turtle.increase_angle();
    //    }))
    //    .add_rule(Rule('+', String::from('+'), |_, turtle, _| {
    //        turtle.decrease_angle();
    //    }))
    //    .add_rule(Rule('[', String::from('['), |_, turtle, _| {
    //        turtle.push_state();
    //    }))
    //    .add_rule(Rule(']', String::from(']'), |_, turtle, _| {
    //        turtle.pop_state();
    //    }));
    //let mut lsys = LSystem::new("F", Turtle::new(pt2(0.0, 0.0), 0.0, 90.0, 5.0), 4)
    //    .add_rule(Rule('F', String::from("F+F-F-F+F"), |draw, turtle, index| {
    //    //.add_rule(Rule('F', String::from("F+F-F-F+F-"), |draw, turtle, index| {
    //        let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
    //        let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
    //        let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
    //        let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
    //        turtle.forward();
    //        turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .color(WHITE)
    //            .weight(1.0);
    //        //draw.ellipse()
    //        //    .x_y(turtle.previous.x, turtle.previous.y)
    //        //    .color(WHITE)
    //        //    .resolution(4.0)
    //        //    .radius(1.0);
    //    }))
    //    .add_rule(Rule('+', String::from("+"), |_, turtle, _| {
    //        turtle.increase_angle();
    //    }))
    //    .add_rule(Rule('-', String::from("-"), |_, turtle, _| {
    //        turtle.decrease_angle();
    //    }));
    let mut lsys = LSystem::new("A", 8, pt2(-240.0, -150.0), 0.0, 60.0, 5.0)
        .add_rule(Rule('A', String::from("B-A-B"), |draw, turtle, index| {
            let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
            let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
            let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
            let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
            turtle.forward();
            turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
            //if turtle.position.distance(pt2(0.0, 0.0)) < 150.0 {
            if turtle.position.distance(pt2(0.0, 0.0)) < RADIUS
                && turtle.previous.distance(pt2(0.0, 0.0)) < RADIUS
            {
                draw.line()
                    .start(turtle.previous)
                    .end(turtle.position)
                    .color(DARKSLATEGRAY)
                    .weight(2.0);
            }
        }))
        .add_rule(Rule('B', String::from("A+B+A"), |draw, turtle, index| {
            let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
            let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
            let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
            let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
            turtle.forward();
            turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
            if turtle.position.distance(pt2(0.0, 0.0)) < RADIUS
                && turtle.previous.distance(pt2(0.0, 0.0)) < RADIUS
            {
                draw.line()
                    .start(turtle.previous)
                    .end(turtle.position)
                    .color(DARKSLATEGRAY)
                    .weight(2.0);
            }
        }))
        .add_rule(Rule('+', String::from("+"), |_, turtle, _| {
            turtle.increase_angle();
        }))
        .add_rule(Rule('-', String::from("-"), |_, turtle, _| {
            turtle.decrease_angle();
        }));
    //let mut lsys = LSystem::new("A", Turtle::new(pt2(0.0, 0.0), 90.0, 25.0, 5.0), 5)
    //    .add_rule(Rule('A', String::from("B[-A-]B"), |draw, turtle, index| {
    //        let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
    //        let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
    //        let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
    //        let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
    //        turtle.forward();
    //        turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .weight(1.0)
    //            .color(WHITE);
    //    }))
    //    .add_rule(Rule('B', String::from("A[+B+]A"), |draw, turtle, index| {
    //        let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
    //        let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
    //        let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
    //        let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
    //        turtle.forward();
    //        turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .weight(1.0)
    //            .color(WHITE);
    //    }))
    //    .add_rule(Rule('+', String::from("+"), |_, turtle, _| {
    //        turtle.increase_angle();
    //    }))
    //    .add_rule(Rule('-', String::from("-"), |_, turtle, _| {
    //        turtle.decrease_angle();
    //    }))
    //    .add_rule(Rule('[', String::from("["), |_, turtle, _| {
    //        turtle.push_state();
    //    }))
    //    .add_rule(Rule(']', String::from("]"), |_, turtle, _| {
    //        turtle.pop_state();
    //    }));
    lsys.build();
    //println!("{}", lsys.string);
    Model { lsys }
}

const RADIUS: f32 = 200.0;

fn update(_app: &App, model: &mut Model, _update: Update) {
    for noise in &mut model.lsys.noise {
        noise.0 += 0.025;
        noise.1 += 0.025;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    let fade = lin_srgba(0.0, 0.0, 0.0, 0.05);
    let beige = lin_srgba(
        BEIGE.red as f32 / 255.0,
        BEIGE.green as f32 / 255.0,
        BEIGE.blue as f32 / 255.0,
        0.05,
    );
    let blend = fade.overlay(beige);
    draw.ellipse().x_y(0.0, 0.0).radius(RADIUS).color(blend);
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(bounds.w(), bounds.h())
        .color(beige);
    model.lsys.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
