
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
    //let mut lsys = LSystem::new('X', Turtle::new(pt2(0.0, 0.0), 90.0, 25.0, 5.0), 5)
    //    .add_alphabet(vec!['X', 'F', '-', '+', '[', ']'])
    //    .add_rule(Rule(
    //        'X',
    //        //String::from("F+[[X]-X]-F[-FX]+X"),
    //        String::from("F+[[X]-X]-F-[FX]+X"),
    //        |_draw, _turtle| {},
    //    ))
    //    .add_rule(Rule('F', String::from("FF"), |draw, turtle| {
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .weight(1.0)
    //            .color(WHITE);
    //        turtle.forward();
    //    }))
    //    .add_rule(Rule('-', String::from('-'), |_draw, turtle| {
    //        turtle.increase_angle();
    //    }))
    //    .add_rule(Rule('+', String::from('+'), |_draw, turtle| {
    //        turtle.decrease_angle();
    //    }))
    //    .add_rule(Rule('[', String::from('['), |_draw, turtle| {
    //        turtle.push_state();
    //    }))
    //    .add_rule(Rule(']', String::from(']'), |_draw, turtle| {
    //        turtle.pop_state();
    //    }));
    //let mut lsys = LSystem::new('F', Turtle::new(pt2(0.0, 0.0), 0.0, 90.0, 5.0), 4)
    //    .add_alphabet(vec!['F', '-', '+'])
    //    .add_rule(Rule('F', String::from("F+F-F-F+F"), |draw, turtle| {
    //    //.add_rule(Rule('F', String::from("F+F-F-F+F-"), |draw, turtle| {
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
    //        turtle.forward();
    //    }))
    //    .add_rule(Rule('+', String::from("+"), |_draw, turtle| {
    //        turtle.increase_angle();
    //    }))
    //    .add_rule(Rule('-', String::from("-"), |_draw, turtle| {
    //        turtle.decrease_angle();
    //    }));
    let mut lsys = LSystem::new('A', Turtle::new(pt2(0.0, 0.0), 0.0, 60.0, 5.0), 6)
        .add_alphabet(vec!['A', 'B', '-', '+'])
        .add_rule(Rule('A', String::from("B-A-B"), |draw, turtle| {
            draw.line()
                .start(turtle.previous)
                .end(turtle.position)
                .color(WHITE)
                .weight(1.0);
            turtle.forward();
        }))
        .add_rule(Rule('B', String::from("A+B+A"), |draw, turtle| {
            draw.line()
                .start(turtle.previous)
                .end(turtle.position)
                .color(WHITE)
                .weight(1.0);
            turtle.forward();
        }))
        .add_rule(Rule('+', String::from("+"), |_draw, turtle| {
            turtle.increase_angle();
        }))
        .add_rule(Rule('-', String::from("-"), |_draw, turtle| {
            turtle.decrease_angle();
        }));
    lsys.build();
    Model { lsys }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.lsys.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
