use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou_sketches::lsystem::{LSystem, Rule, Turtle};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    lsys: LSystem,
}

impl Default for Settings {
    fn default() -> Self {
        //let mut lsys = LSystem::new("X", Turtle::new(pt2(0.0, 0.0), 90.0, 25.0, 5.0), 5)
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
        let mut lsys = LSystem::new("F", 4, pt2(-150.0, 0.0), 0.0, 90.0, 5.0)
            .add_rule(Rule(
                'F',
                String::from("F+F-F-F+F"),
                |draw, turtle, _index| {
                    //.add_rule(Rule('F', String::from("F+F-F-F+F-"), |draw, turtle| {
                    if pt2(0.0, 0.0).distance(turtle.previous) <= 150.0 {
                        draw.line()
                            .start(turtle.previous)
                            .end(turtle.position)
                            .color(WHITE)
                            .weight(1.0);
                    }
                    //draw.ellipse()
                    //    .x_y(turtle.previous.x, turtle.previous.y)
                    //    .color(WHITE)
                    //    .resolution(4.0)
                    //    .radius(1.0);
                    turtle.forward();
                },
            ))
            .add_rule(Rule('+', String::from("+"), |_draw, turtle, _index| {
                turtle.increase_angle();
            }))
            .add_rule(Rule('-', String::from("-"), |_draw, turtle, _index| {
                turtle.decrease_angle();
            }));
        //let mut lsys = LSystem::new('A', Turtle::new(pt2(-150.0, -50.0), 0.0, 60.0, 5.0), 5)
        //    .add_rule(Rule('A', String::from("B-A-B"), |draw, turtle| {
        //        if pt2(0.0, 0.0).distance(turtle.previous) <= 150.0 {
        //            draw.line()
        //                .start(turtle.previous)
        //                .end(turtle.position)
        //                .color(WHITE)
        //                .weight(1.0);
        //        }
        //        turtle.forward();
        //    }))
        //    .add_rule(Rule('B', String::from("A+B+A"), |draw, turtle| {
        //        if pt2(0.0, 0.0).distance(turtle.previous) <= 150.0 {
        //            draw.line()
        //                .start(turtle.previous)
        //                .end(turtle.position)
        //                .color(WHITE)
        //                .weight(1.0);
        //        }
        //        turtle.forward();
        //    }))
        //    .add_rule(Rule('+', String::from("+"), |_draw, turtle| {
        //        turtle.increase_angle();
        //    }))
        //    .add_rule(Rule('-', String::from("-"), |_draw, turtle| {
        //        turtle.decrease_angle();
        //    }));
        lsys.build();
        Self { lsys }
    }
}

struct Model {
    egui: Egui,
    settings: Settings,
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(600, 600)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);
    Model {
        egui,
        settings: Settings::default(),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    let mut clicked = false;
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Depth:");
        ui.add(egui::Slider::new(&mut model.settings.lsys.depth, 1..=25));

        ui.label("Angle:");
        ui.add(egui::Slider::new(
            &mut model.settings.lsys.turtle.angle_step,
            0.0..=360.0,
        ));

        ui.label("Distance:");
        ui.add(egui::Slider::new(
            &mut model.settings.lsys.turtle.distance,
            0.0..=50.0,
        ));

        ui.label("X:");
        ui.add(egui::Slider::new(
            &mut model.settings.lsys.turtle.position.x,
            -600.0..=600.0,
        ));

        //ui.label("Map Output Min");
        //ui.add(egui::Slider::new(&mut settings.out_min, -10.0..=1000.0));

        //ui.label("Map Output Max");
        //ui.add(egui::Slider::new(&mut settings.out_max, -10.0..=1000.0));

        //ui.label("Weight:");
        //ui.add(egui::Slider::new(&mut settings.weight, 0.0..=100.0));

        // Random color button
        clicked = ui.button("Reset").clicked();
    });
    let settings = &mut model.settings;
    if clicked {
        //*settings = Settings::default();
        settings.lsys.build();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    draw.ellipse().x_y(0.0, 0.0).radius(150.0).color(PLUM);

    model.settings.lsys.draw(&draw);

    draw.to_frame(app, &frame).unwrap();

    model.egui.draw_to_frame(&frame).unwrap();
}
