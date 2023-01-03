use nannou::geom::Rect;
use nannou::prelude::*;
use nannou::text::{font::from_file, Font};
use nannou_sketches::grad_circle::Background;

fn main() {
    nannou::app(model).update(update).run();
}

/*#[rustfmt::skip]
const CODE: &str =
" 1 struct Depth;
 2
 3 struct Adjacency;
 4
 5 fn main() {
 6    let counter = 0xdead;
 7    println!(\"{}\", counter);
 8    println!(\"Imposter\");
 9 }
10
11 struct Color;";*/

#[rustfmt::skip]
const CODE: &str = 
"struct Depth;

struct Adjacency;

fn main() {
   let counter = 0xdead;
   println!(\"{}\", counter);
   println!(\"Imposter\");
}

struct Color;";

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

    fn draw(&self, draw: &Draw) {
        let string =
            String::from_utf8(self.string.as_bytes()[0..self.string_index].to_vec()).unwrap();
        let text = text(&string)
            .font_size(self.font_size)
            .left_justify()
            .align_bottom()
            .font(self.font.clone())
            .build(self.rect);
        draw.path().fill().color(WHITE).events(text.path_events());
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
    background: Background,
    writer: CodeWriter,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let font = from_file("assets/font.ttf").unwrap();
    let r = Rect::from_x_y_w_h(0.0, 0.0, 300.0, 300.0);
    let writer = CodeWriter::new(font, 13, r, String::from(CODE));
    Model {
        background: Background::new(5, 0.000125, 50.0),
        writer,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.writer.increment();
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
    model.background.draw(&draw);
    model.writer.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
