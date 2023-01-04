//! Import photo and create a circular wave ((()))
//! of moving "glitch"
use nannou::prelude::*;
use nannou::image;
use nannou::image::GenericImageView;
use nannou::color::rgb_u32;

fn main() {
    nannou::app(model).update(update).run();
}

struct GlitchCircle {
    points: Vec<Vec2>,
    radius: f32,
}

impl GlitchCircle {
    fn new(radius: f32) -> Self {
        let mut points = Vec::<Vec2>::new();
        let mut t = 0.0;
        while t <= 2.0 * PI {
            let x = t.cos() * radius;
            let y = t.sin() * radius;
            points.push(pt2(x, y));
            t += 0.01;
        }
        Self { points, radius }
    }
    fn draw(&self, draw: &Draw, image: &image::DynamicImage, win: Rect) {
        //draw.polyline().points(self.points.clone()).color(WHITE);
        let (w, h) = image.dimensions();
        for grid_x in 0..w {
            for grid_y in 0..h {
                // get current color
                let c = image.get_pixel(grid_x, grid_y);
                let tile_width = win.w() / w as f32;
                let tile_height = win.h() / h as f32;
                let pos_x = win.left() + tile_width * grid_x as f32 + (tile_width / 2.0);
                let pos_y = win.top() - tile_height * grid_y as f32 - (tile_height / 2.0);
                for point in &self.points {
                    draw.ellipse()
                        .color(WHITE)
                        .radius(5.0)
                        .x_y(point.x, point.y);
                }
                draw.rect()
                    .color(rgb8(c[0], c[1], c[2]))
                    .w_h(tile_width, tile_height)
                    .x_y(pos_x, pos_y);

            }
        }
    }
}

struct Model {
    glitch: GlitchCircle,
    image: image::DynamicImage,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let assets = app.assets_path().unwrap();
    let img_path = assets
        .join("images")
        .join("stealyourface.png");

    let image = image::open(img_path).unwrap();
    Model {
        glitch: GlitchCircle::new(100.0),
        image
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    model.glitch.draw(&draw, &model.image, win);
    draw.to_frame(app, &frame).unwrap();
}
