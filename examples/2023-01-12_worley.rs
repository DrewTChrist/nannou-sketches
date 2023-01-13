use nannou::image::RgbaImage;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use std::cmp::Ordering::Equal;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<(Vec2, f32, f32)>,
    perlin: Perlin,
    texture: wgpu::Texture,
    image: RgbaImage,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(300, 300).view(view).build().unwrap();
    let window = app.main_window();
    let bounds = app.window_rect();
    let image = RgbaImage::new(bounds.w() as u32, bounds.h() as u32);
    let texture = wgpu::TextureBuilder::new()
        .size([bounds.w() as u32, bounds.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());
    let mut points: Vec<(Vec2, f32, f32)> = Vec::new();
    let perlin = Perlin::new();
    for _ in 0..5 {
        points.push((
            pt2(
                random_range(bounds.left(), bounds.right()),
                random_range(bounds.bottom(), bounds.top()),
            ),
            random_range(-500.0, 500.0),
            random_range(-500.0, 500.0),
        ));
    }
    Model {
        points,
        perlin,
        texture,
        image,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    for x in 0..bounds.w() as u32 {
        for y in 0..bounds.h() as u32 {
            let mut distances = Vec::new();
            for (point, _, _) in &model.points {
                let dist =
                    pt2(x as f32 - bounds.w() / 2.0, y as f32 - bounds.h() / 2.0).distance(*point);
                distances.push(dist);
            }
            let n = 0;
            distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
            let noise = map_range(distances[n], 0.0, bounds.w() / 5.0, 255.0, 0.0) as u8;
            model.image.put_pixel(
                x,
                (bounds.h() as u32 - y) - 1,
                nannou::image::Rgba([noise, noise, noise, 255]),
            );
        }
    }
    for (point, xstep, ystep) in &mut model.points {
        let x_noise = map_range(
            model.perlin.get([*xstep as f64 * 0.1, 0.0]),
            -1.0,
            1.0,
            -5.0,
            5.0,
        );
        let y_noise = map_range(
            model.perlin.get([*ystep as f64 * 0.1, 0.0]),
            -1.0,
            1.0,
            -5.0,
            5.0,
        );
        point.x += x_noise;
        point.y += y_noise;
        *xstep += 0.003;
        *ystep += 0.003;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    let flat_samples = model.image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );
    draw.texture(&model.texture);
    //for (point, _, _) in &model.points {
    //    draw.ellipse().x_y(point.x, point.y).radius(5.0).color(RED);
    //}
    draw.to_frame(app, &frame).unwrap();
}
