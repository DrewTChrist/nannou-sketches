use nannou::image::{ImageBuffer, RgbaImage};
use nannou::prelude::*;
use std::cmp::Ordering::Equal;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Vec2>,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let window = app.main_window();
    let bounds = app.window_rect();
    let texture = wgpu::TextureBuilder::new()
        .size([bounds.w() as u32, bounds.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());
    let mut points: Vec<Vec2> = Vec::new();
    for _ in 0..3 {
        points.push(pt2(
            random_range(bounds.left(), bounds.right()),
            random_range(bounds.bottom(), bounds.top()),
        ));
    }
    Model { points, texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let bounds = app.window_rect();
    let draw = app.draw();
    //draw.background().color(BLACK);
    frame.clear(BLACK);
    let mut image = RgbaImage::new(bounds.w() as u32, bounds.h() as u32);
    for x in 0..bounds.w() as u32 {
        for y in 0..bounds.h() as u32 {
            let mut distances = Vec::new();
            for point in &model.points {
                let dist =
                    pt2(x as f32 - bounds.w() / 2.0, y as f32 - bounds.h() / 2.0).distance(*point);
                distances.push(dist);
            }
            let n = 0;
            distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
            let noise = map_range(distances[n], 0.0, bounds.w() / 5.0, 255.0, 0.0) as u8;
            image.put_pixel(x, y, nannou::image::Rgba([noise, noise, noise, 255]));
        }
    }
    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );
    draw.texture(&model.texture).rotate(-PI);
    for point in &model.points {
        draw.ellipse().x_y(point.x, point.y).radius(10.0).color(RED);
    }
    draw.to_frame(app, &frame).unwrap();
}
