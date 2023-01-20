use nannou::color::Blend;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use nannou_sketches::capturer::FrameCapturer;
use nannou_sketches::lsystem::{LSystem, Rule, Turtle};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    lsys1: LSystem,
    lsys2: LSystem,
    capturer: FrameCapturer,
}

const TEXTURE_SIZE: [u32; 2] = [600, 600];

fn model(app: &App) -> Model {
    let [win_w, win_h] = [TEXTURE_SIZE[0], TEXTURE_SIZE[1]];
    let window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let window = app.window(window_id).unwrap();
    let mut lsys1 = LSystem::new("A", 8, pt2(-240.0, -150.0), 0.0, 60.0, 5.0)
        .add_rule(Rule('A', String::from("B-A-B"), |draw, turtle, index| {
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
                    .color(BEIGE)
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
                    .color(BEIGE)
                    .weight(2.0);
            }
        }))
        .add_rule(Rule('+', String::from("+"), |_, turtle, _| {
            turtle.increase_angle();
        }))
        .add_rule(Rule('-', String::from("-"), |_, turtle, _| {
            turtle.decrease_angle();
        }));
    lsys1.build();
    let mut lsys2 = lsys1.clone();
    lsys2.rules.remove(0);
    lsys2.rules.remove(0);
    lsys2 = lsys2
        .add_rule(Rule('A', String::from("B-A-B"), |draw, turtle, index| {
            let x_noise = turtle.perlin.get([turtle.noise[index].0, 0.0]);
            let y_noise = turtle.perlin.get([turtle.noise[index].1, 0.0]);
            let xnmap = map_range(x_noise, -1.0, 1.0, -2.5, 2.5);
            let ynmap = map_range(y_noise, -1.0, 1.0, -2.5, 2.5);
            turtle.forward();
            turtle.position = pt2(turtle.position.x + xnmap, turtle.position.y + ynmap);
            if turtle.position.distance(pt2(0.0, 0.0)) > RADIUS
                && turtle.previous.distance(pt2(0.0, 0.0)) > RADIUS
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
            if turtle.position.distance(pt2(0.0, 0.0)) > RADIUS
                && turtle.previous.distance(pt2(0.0, 0.0)) > RADIUS
            {
                draw.line()
                    .start(turtle.previous)
                    .end(turtle.position)
                    .color(DARKSLATEGRAY)
                    .weight(2.0);
            }
        }));
    //println!("{}", lsys.string);
    std::fs::create_dir_all(&capture_directory(app)).unwrap();
    Model {
        lsys1,
        lsys2,
        capturer: FrameCapturer::new(&window, TEXTURE_SIZE),
    }
}

const RADIUS: f32 = 200.0;

fn update(app: &App, model: &mut Model, _update: Update) {
    let draw = &model.capturer.draw;
    draw.reset();
    for noise in &mut model.lsys1.noise {
        noise.0 += 0.025;
        noise.1 += 0.025;
    }
    for noise in &mut model.lsys2.noise {
        noise.0 += 0.025;
        noise.1 += 0.025;
    }
    let bounds = app.window_rect();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    let fade = lin_srgba(
        BLACK.red as f32 / 255.0,
        BLACK.green as f32 / 255.0,
        BLACK.blue as f32 / 255.0,
        0.1,
    );
    let beige = lin_srgba(
        BEIGE.red as f32 / 255.0,
        BEIGE.green as f32 / 255.0,
        BEIGE.blue as f32 / 255.0,
        0.05,
    );
    let blend = beige.darken(fade);
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(bounds.w(), bounds.h())
        .color(beige);
    draw.ellipse().x_y(0.0, 0.0).radius(RADIUS).color(fade);
    model.lsys1.draw(&draw);
    model.lsys2.draw(&draw);
    model.capturer.capture(&app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut encoder = frame.command_encoder();
    model
        .capturer
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

fn exit(app: &App, model: Model) {
    let window = app.main_window();
    let device = window.device();
    model
        .capturer
        .texture_capturer
        .await_active_snapshots(&device)
        .unwrap();
}

fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}
