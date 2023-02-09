use nannou::color::Alpha;
use nannou::prelude::*;
use nannou_sketches::{gradient::grad_many, utilities::u32_to_srgba};
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct Circle {
    location: Vec2,
    radius: f32,
    color: Alpha<Rgb<f32>, f32>,
}

struct Settings {
    t_step: f32,
    circ_rad_step: f32,
    swirl_rad_step: f32,
    color_t_step: f32,
    range: f32
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            t_step: 0.1,
            circ_rad_step: 0.1,
            swirl_rad_step: 0.5,
            color_t_step: 0.005,
            range: 12.0 * PI,
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

struct Model {
    colors: Vec<Alpha<Rgb<f32>, f32>>,
    color_t: Vec<f32>,
    egui: Egui,
    settings: Settings
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
    let colors: Vec<Alpha<Rgb<f32>, f32>> = vec![0xa7d2cb, 0xf2d388, 0xc98474, 0x874c62, 0xa7d2cb]
        .iter()
        .map(|c| u32_to_srgba(*c, 1.0))
        .collect();
    let mut color_t = Vec::new();
    let mut t = 0.0;
    let mut ct = 0.0;
    while t < 12.0 * PI {
        color_t.push(ct);
        t += 0.1;
        ct += 0.002;
    }
    Model { colors, color_t, egui, settings: Settings::default() }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("circ_rad_step");
        ui.add(egui::Slider::new(&mut settings.circ_rad_step, 0.0..=1.0));

        ui.label("color_t_step");
        ui.add(egui::Slider::new(&mut settings.color_t_step, 0.001..=1.0));

        ui.label("range");
        ui.add(egui::Slider::new(&mut settings.range, 1.0..=500.0));

        ui.label("swirl_rad_step");
        ui.add(egui::Slider::new(&mut settings.swirl_rad_step, 0.0..=500.0));

        ui.label("t_step");
        ui.add(egui::Slider::new(&mut settings.t_step, 0.0..=10.0));
    });
    if model.color_t[0] < 1.0 {
        model.color_t[0] += 0.005;
    } else {
        model.color_t[0] = 0.0;
    }
    //for i in 0..model.color_t.len() {
    //    if model.color_t[i] < 1.0 {
    //        model.color_t[i] += 0.005;
    //    } else {
    //        model.color_t[i] = 0.0;
    //    }
    //}
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _bounds = app.window_rect();
    let draw = app.draw();
    draw.background().color(BEIGE);
    //let t_step = 0.25;
    let t_step = 0.1;
    let circ_rad_step = 0.1;
    let swirl_rad_step = 0.5;
    let color_t_step = 0.002;
    let range = 12.0 * PI;
    let mut t = 0.0;
    let mut swirl_radius = 1.0;
    let mut circle_radius = 5.0;
    let mut color_t = Vec::new();
    let mut c_t_count = 0.0;
    let mut ct = 0.0;
    while c_t_count < model.settings.range {
        color_t.push(ct);
        c_t_count += model.settings.t_step;
        ct += model.settings.color_t_step;
    }
    let mut index: usize = 0;
    while t <= model.settings.range {
        let x = t.cos() * swirl_radius;
        let y = t.sin() * swirl_radius;
        draw.ellipse()
            .xy(pt2(x, y))
            //.color(grad_many(&model.colors, model.color_t[index]))
            .color(grad_many(&model.colors, color_t[index]))
            .radius(circle_radius);
        t += model.settings.t_step;
        circle_radius += model.settings.circ_rad_step;
        swirl_radius += model.settings.swirl_rad_step;
        index += 1;
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
