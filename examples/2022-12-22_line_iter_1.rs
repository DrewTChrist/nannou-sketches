use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct NoiseLine {
    perlin: Perlin,
    points: Vec<Vec2>,
    noises: Vec<f32>,
    weight: f32,
    height: f32,
}

impl NoiseLine {
    fn new(weight: f32, height: f32) -> Self {
        let mut points = Vec::<Vec2>::new();
        let mut noises = Vec::<f32>::new();
        for i in (-300..300).step_by(1) {
            points.push(pt2(i as f32, height));
            noises.push(random_range(-250.0, 250.0));
        }
        Self {
            perlin: Perlin::new(),
            points,
            noises,
            weight,
            height,
        }
    }
    fn draw(&self, draw: &Draw) {
        draw.polyline()
            .color(STEELBLUE)
            .weight(self.weight)
            .points(self.points.clone());
    }
}

struct Settings {
    lines: Vec<NoiseLine>,
    scale: [f64; 2],
    in_min: f64,
    in_max: f64,
    out_min: f64,
    out_max: f64,
}

struct Model {
    egui: Egui,
    settings: Settings,
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

impl Settings {
    fn default() -> Self {
        let mut lines = Vec::<NoiseLine>::new();

        let mut height: f32 = 300.0;
        for i in 0..15 {
            lines.push(NoiseLine::new(i as f32, height));
            height -= 35.0 + (i as f32) * 2.0;
        }
        Self {
            lines,
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

    Model {
        egui,
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
        ui.add(egui::Slider::new(&mut settings.scale[0], 0.001..=1.0));

        ui.label("Map Input Min:");
        ui.add(egui::Slider::new(&mut settings.in_min, -10.0..=10.0));

        ui.label("Map Input Max");
        ui.add(egui::Slider::new(&mut settings.in_max, -10.0..=10.0));

        ui.label("Map Output Min");
        ui.add(egui::Slider::new(&mut settings.out_min, -100.0..=1000.0));

        ui.label("Map Output Max");
        ui.add(egui::Slider::new(&mut settings.out_max, -100.0..=1000.0));

        clicked = ui.button("Reset").clicked();
    });

    if clicked {
        *settings = Settings::default();
    }

    for line in settings.lines.iter_mut() {
        for (idx, n) in line.noises.iter_mut().enumerate() {
            let noise = line.perlin.get([
                (app.elapsed_frames() as f64 + idx as f64) * settings.scale[0],
                0.0,
            ]);
            let noise_mapped = map_range(
                noise,
                settings.in_min,
                settings.in_max,
                settings.out_min,
                settings.out_max,
            );
            *n = noise_mapped as f32;
            line.points[idx].y = noise_mapped as f32 + line.height;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    for line in &model.settings.lines {
        line.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
