use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    line: Vec<Vec2>,
    noise_x_y: [f64; 2],
    scale: [f64; 2],
    in_min: f64,
    in_max: f64,
    out_min: f64,
    out_max: f64,
}

struct Model {
    egui: Egui,
    noise: Perlin,
    settings: Settings,
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

impl Settings {
    fn default() -> Self {
        let mut line = Vec::<Vec2>::new();
        //for i in (-250..250).step_by(2) {
        for i in -250..250 {
            line.push(pt2(i as f32, -50.0));
        }
        Self {
            line,
            noise_x_y: [1.0, 1.0],
            //scale: [0.01, 0.06]
            scale: [0.01, 0.01],
            in_min: -1.0,
            in_max: 1.0,
            out_min: -1.0,
            out_max: 1.0,
        }
    }
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

    let noise = Perlin::new();
    Model {
        egui,
        noise,
        settings: Settings::default(),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let mut clicked: bool = false;

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Noise Scale X:");
        ui.add(egui::Slider::new(&mut settings.scale[0], 0.001..=10.0));

        ui.label("Noise Scale Y:");
        ui.add(egui::Slider::new(&mut settings.scale[1], 0.001..=10.0));

        ui.label("Map Input Min:");
        ui.add(egui::Slider::new(&mut settings.in_min, -10.0..=10.0));

        ui.label("Map Input Max");
        ui.add(egui::Slider::new(&mut settings.in_max, -10.0..=10.0));

        ui.label("Map Output Min");
        ui.add(egui::Slider::new(&mut settings.out_min, -10.0..=10.0));

        ui.label("Map Output Max");
        ui.add(egui::Slider::new(&mut settings.out_max, -10.0..=10.0));

        // Random color button
        clicked = ui.button("Reset").clicked();
    });
    if clicked {
        *settings = Settings::default();
    }
    for point in settings.line.iter_mut() {
        //let noise = map_range(model.noise.get([model.noise_x_y[0], model.noise_x_y[1]]), -1.0, 1.0, -5.0, 5.0);
        //let noise = map_range(model.noise.get([point.x as f64 / model.noise_x_y[0], point.y as f64 / model.noise_x_y[1]]), -1.0, 1.0, -5.0, 5.0);
        //let noise = model.noise.get([point.x as f64 * model.scale[0], point.y as f64 * model.scale[1]]);
        let noise = map_range(
            model.noise.get([
                point.x as f64 * settings.scale[0],
                point.y as f64 * settings.scale[1],
            ]),
            //-1.0,
            //1.0,
            //-1.0,
            //10.0,
            settings.in_min,
            settings.in_max,
            settings.out_min,
            settings.out_max,
        );
        point.y += noise as f32;
        //model.noise_x_y[0] += 0.003;
        //model.noise_x_y[1] += 0.003;
        //model.noise_x_y[0] += 0.0009;
        //model.noise_x_y[1] += 0.0009;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    draw.polyline()
        .color(STEELBLUE)
        .weight(2.0)
        .points(model.settings.line.clone());

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
