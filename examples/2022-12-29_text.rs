use nannou::geom::Rect;
use nannou::prelude::*;
use nannou::text::{font::from_file, Font};
use nannou_sketches::grad_circle::Background;

fn main() {
    nannou::app(model).update(update).run();
}

#[rustfmt::skip]
const CODE: &str = 
"\
 1 struct Depth;
 2
 3 struct Adjacency;
 4
 5 fn main() {
 6    let counter = 0xdead;
 7    println!(\"{}\", counter);
 8    println!(\"Imposter\");
 9 }
10
11 struct Color;";

struct CodeWriter {
    font: Font,
    font_size: u32,
    rect: Rect,
    string: String,
    lines: Vec<String>,
    index: usize,
    line: usize,         // current line
    line_index: usize,   // current character in line
    string_index: usize, // current spot in total string
}

impl CodeWriter {
    /// Prefer a mono space font for code
    fn new(font: Font, font_size: u32, rect: Rect, string: String) -> Self {
        Self {
            font,
            font_size,
            rect,
            index: 0,
            string_index: 0,
            line: 0,
            line_index: 0,
            lines: string.split("\n").map(|s| String::from(s)).collect(),
            //string,
            string: String::new(),
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
        if self.line_index == self.lines[self.line].len() - 1 {
            if self.line + 1 < self.lines.len() {
                self.line += 1;
                self.line_index = 0;
            }
        } else if self.line_index + step < self.lines[self.line].len() {
            self.line_index += step;
            self.string_index += step;
            //self.string.push(self.lines[self.line].as_bytes()[self.line_index] as char);
            self.string.push_str(
                &String::from_utf8(
                    self.lines[self.line].as_bytes()[self.line_index - step..self.line_index]
                        .to_vec(),
                )
                .unwrap(),
            );
        }
        //self.string_index += step;
        //println!(
        //    "Line: {} Col: {} str: {} Num Lines: {}",
        //    self.line,
        //    self.line_index,
        //    self.string_index,
        //    self.lines[self.line].len()
        //);
    }

    /*fn draw(&self, draw: &Draw) {
        let string = String::from_utf8(self.string.as_bytes()[0..self.index].to_vec()).unwrap();
        let text = text(&string)
            .font_size(self.font_size)
            .left_justify()
            .align_bottom()
            .font(self.font.clone())
            .build(self.rect);
        draw.path().fill().color(WHITE).events(text.path_events());
    }

    fn increment(&mut self) {
        let mut tries = 0;
        let mut rand = random_range(0, 3);
        while !(self.index + rand < self.string.len()) && tries < 3 {
            rand = random_range(0, 3);
            tries += 1;
        }
        if self.index + rand < self.string.len() {
            self.index += rand;
        } else if self.index + 1 < self.string.len() {
            self.index += 1;
        }
    }*/
}

struct Model {
    background: Background,
    writer: CodeWriter,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let font = from_file("font.ttf").unwrap();
    let r = Rect::from_x_y_w_h(0.0, 0.0, 300.0, 300.0);
    Model {
        background: Background::new(5, 0.000125, 50.0),
        writer: CodeWriter::new(font, 13, r, String::from(CODE)),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.writer.increment();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.background.draw(&draw);
    model.writer.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
