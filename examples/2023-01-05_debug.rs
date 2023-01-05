use nannou::geom::Rect;
use nannou::prelude::*;
use nannou::text::{font::from_file, Font};
use nannou_sketches::grad_circle::Background;
use nannou::color::rgb_u32;

fn main() {
    nannou::app(model).update(update).run();
}

#[rustfmt::skip]
const CODE: &str = 
"
fn draw(&self, draw: &Draw) {
    let mut x = (self.a - self.b) * self.t.cos() + self.h 
            * (((self.a - self.b) / self.b) * self.t).cos();
    let mut y = (self.a - self.b) * self.t.sin() + self.h 
            * (((self.a - self.b) / self.b) * self.t).sin();
    x *= self.scale;
    y *= self.scale;
    draw.ellipse()
        .x_y(x + self.x, y + self.y)
        .w_h(10.0, 10.0)
        .color(self.color);
}";

struct Hypotrochoid {
    x: f32,
    y: f32,
    a: f32,
    b: f32,
    h: f32,
    t: f32,
    scale: f32,
    color: Rgb<u8>,
}

impl Hypotrochoid {
    fn new(x: f32, y: f32, a: f32, b: f32, h: f32, scale: f32, color: Rgb<u8>) -> Self {
        Self {
            x,
            y,
            a,
            b,
            h,
            t: 0.0,
            scale,
            color,
        }
    }

    fn draw(&self, draw: &Draw) -> (f32, f32) {
        let mut x = (self.a - self.b) * self.t.cos()
            + self.h * (((self.a - self.b) / self.b) * self.t).cos();
        let mut y = (self.a - self.b) * self.t.sin()
            + self.h * (((self.a - self.b) / self.b) * self.t).sin();
        x *= self.scale;
        y *= self.scale;
        draw.ellipse()
            .x_y(x + self.x, y + self.y)
            .w_h(10.0, 10.0)
            .color(self.color);
        (x, y)
    }
}

struct CodeWriter {
    font: Font,
    font_size: u32,
    rect: Rect,
    string: String,
    lines: Vec<String>,
    line: usize,         // current line
    line_index: usize,   // current character in line
    string_index: usize, // current spot in total string
    finished: bool,
    line_num: bool,
}

impl CodeWriter {
    /// Prefer a mono space font for code
    fn new(font: Font, font_size: u32, rect: Rect, string: String) -> Self {
        Self {
            font,
            font_size,
            rect,
            string_index: 0,
            line: 0,
            line_index: 0,
            lines: string.split('\n').map(String::from).collect(),
            string: String::new(),
            finished: false,
            line_num: false,
        }
    }

    fn draw(&self, draw: &Draw, x: f32, y: f32) {
        let mut string =
            String::from_utf8(self.string.as_bytes()[0..self.string_index].to_vec()).unwrap();
        let mut x_string = String::from("(");
        x_string.push_str(x.to_string();
        let mut y_string = y.to_string();
        string = string.replace("(x ", x_string);
        string = string.replace(", y ", y_string);
        let text = text(&string)
            .font_size(self.font_size)
            .left_justify()
            .align_bottom()
            .font(self.font.clone())
            .build(self.rect);
        draw.path().fill().color(WHITE).x_y(0.0, 200.0).events(text.path_events());
    }

    fn increment(&mut self) {
        // if at the end of a line go to next line
        // copy line to total string
        // set column index to zero
        // increase total string index
        let step = random_range(0, 3);
        if !self.finished {
            if (self.line_index == 0 && !self.line_num) || self.lines[self.line].is_empty() {
                let mut line_num = (self.line + 1).to_string();
                if line_num.len() == 1 {
                    line_num.insert(0, ' ');
                }
                line_num.push(' ');
                self.string.push_str(&line_num);
                self.string_index += line_num.len();
                self.line_num = true;
            }
            if self.lines[self.line].is_empty()
                || self.line_index == self.lines[self.line].len() - 1
            {
                let ch = if self.lines[self.line].is_empty() {
                    String::from("")
                } else {
                    String::from(self.lines[self.line].as_bytes()[self.line_index] as char)
                };
                self.string.push_str(&ch);
                self.string.push('\n');
                self.string_index += 1 + ch.len();
                if self.line + 1 < self.lines.len() {
                    self.line += 1;
                    self.line_index = 0;
                    self.line_num = false;
                } else {
                    self.finished = true;
                }
            } else if self.line_index + step < self.lines[self.line].len() {
                self.line_index += step;
                self.string_index += step;
                self.string.push_str(
                    &String::from_utf8(
                        self.lines[self.line].as_bytes()[self.line_index - step..self.line_index]
                            .to_vec(),
                    )
                    .unwrap(),
                );
            }
        }
    }
}

struct Model {
    //background: Background,
    writer: CodeWriter,
    h: Hypotrochoid
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let font = from_file("assets/font.ttf").unwrap();
    let r = Rect::from_x_y_w_h(0.0, 0.0, 600.0, 700.0);
    let writer = CodeWriter::new(font, 13, r, String::from(CODE));
    Model {
        //background: Background::new(5, 0.000125, 50.0),
        writer,
        h: Hypotrochoid::new(0.0, 0.0, 5.0, 7.0, 2.2, 25.0, rgb_u32(0xD7E9B9)),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.writer.increment();
    model.h.t += 0.05;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(600.0, 600.0)
        .color(srgba(0.0, 0.0, 0.0, 0.9));
    //model.background.draw(&draw);
    let (x, y) = model.h.draw(&draw);
    model.writer.draw(&draw, x, y);
    draw.to_frame(app, &frame).unwrap();
}
