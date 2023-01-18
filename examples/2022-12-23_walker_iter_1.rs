use nannou::color::rgb_u32;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Copy, Clone)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

const HEIGHT: isize = 350;
const WIDTH: isize = 350;
const STEP: usize = 5;

impl Direction {
    fn to_row_col(&self, row: i32, col: i32) -> (i32, i32) {
        match self {
            Direction::N => (row + 1, col),
            Direction::NE => (row + 1, col + 1),
            Direction::E => (row, col + 1),
            Direction::SE => (row - 1, col + 1),
            Direction::S => (row - 1, col),
            Direction::SW => (row - 1, col - 1),
            Direction::W => (row, col - 1),
            Direction::NW => (row + 1, col - 1),
        }
    }
}

const DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
    Direction::NE,
    Direction::SE,
    Direction::SW,
    Direction::NW,
];

struct Walker {
    start: Vec2,
    points: Vec<Vec2>,
    grid: Grid,
    current_row: i32,
    current_col: i32,
    perlin: Perlin,
}

impl Walker {
    fn new(start: Vec2, current_row: i32, current_col: i32) -> Self {
        let mut points = Vec::<Vec2>::new();
        points.push(start);
        Self {
            start,
            points,
            grid: Grid::new(HEIGHT, WIDTH, STEP),
            current_row,
            current_col,
            perlin: Perlin::new(),
        }
    }
    fn draw(&self, draw: &Draw) {
        let line_primary = rgb_u32(0x678983);
        let line_secondary = rgb_u32(0xE6DDC4);
        let third = rgb_u32(0xFCF9BE);
        for i in 1..self.points.len() {
            draw.line()
                .start(self.points[i - 1] - 3.0)
                .end(self.points[i] - 3.0)
                .color(third)
                .weight(1.0);
            draw.line()
                .start(self.points[i - 1] - 2.0)
                .end(self.points[i] - 2.0)
                .color(line_secondary)
                .weight(2.0);
            draw.line()
                .start(self.points[i - 1])
                .end(self.points[i])
                .color(line_primary)
                .weight(3.0);
        }
    }
    /* - Make sure next point isn't already in the path
     * - Make sure it doesn't go outside of the grid boundaries */
    fn get_new_point(&mut self, t: f64) {
        let noise = self.perlin.get([t, 0.0]);
        let nmap = map_range(noise, -1.0, 1.0, 0.0, 8.0);
        //println!("{}", nmap as usize);
        let mut rand_direction = DIRECTIONS[nmap as usize];
        let mut new_location = rand_direction.to_row_col(self.current_row, self.current_col);
        while new_location.0 < 0
            || new_location.0 > (HEIGHT as i32 * 2 / STEP as i32) - 1
            || new_location.1 < 0
            || new_location.1 > (WIDTH as i32 * 2 / STEP as i32) - 1
        {
            let r = random_range(-15.0, 15.0);
            let noise = self.perlin.get([t + r, 0.0]);
            let nmap = map_range(noise, -1.0, 1.0, 0.0, 8.0);
            //rand_direction = DIRECTIONS[random_range(0, 8)];
            rand_direction = DIRECTIONS[nmap as usize];
            new_location = rand_direction.to_row_col(self.current_row, self.current_col);
        }
        let next_point = self.grid.points[new_location.0 as usize][new_location.1 as usize];
        //if !self.points.contains(&next_point) {
        self.points.push(next_point);
        self.current_row = new_location.0;
        self.current_col = new_location.1;
        //}
    }
}

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<Vec2>>,
}

impl Grid {
    fn new(height: isize, width: isize, step: usize) -> Self {
        let mut points: Vec<Vec<Vec2>> = Vec::<Vec<Vec2>>::new();
        for i in (-height..height).step_by(step) {
            let mut row: Vec<Vec2> = Vec::<Vec2>::new();
            for j in (-width..width).step_by(step) {
                row.push(pt2(i as f32, j as f32));
            }
            points.push(row);
        }
        Self { points }
    }
    fn draw(&self, draw: &Draw) {
        for row in &self.points {
            for point in row {
                draw.ellipse()
                    .color(BLACK)
                    .x_y(point.x, point.y)
                    .w_h(2.0, 2.0);
            }
        }
    }
}

struct Settings {
    walker: Walker,
    show_grid: bool,
    noise_step: f64,
}

struct Model {
    egui: Egui,
    settings: Settings,
    tick: f64,
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

impl Settings {
    fn default() -> Self {
        let grid = Grid::new(HEIGHT, WIDTH, STEP);
        let num_rows = grid.points.len();
        let num_cols = grid.points[0].len();
        let rand_row = random_range(0, num_rows);
        let rand_col = random_range(0, num_cols);
        let rand_point = grid.points[rand_row][rand_col];
        let walker: Walker = Walker::new(rand_point, rand_row as i32, rand_col as i32);
        Self {
            walker,
            show_grid: false,
            noise_step: 0.1,
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
        tick: random_range(1000.0, 2000.0),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let mut reset: bool = false;
    let mut show_grid: bool = false;

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Noise Step");
        ui.add(egui::Slider::new(&mut settings.noise_step, 0.001..=1.0));

        show_grid = ui.button("Show grid").clicked();

        reset = ui.button("Reset").clicked();
    });

    if show_grid {
        settings.show_grid = !settings.show_grid;
    }

    if reset {
        *settings = Settings::default();
    }

    let _t = app.elapsed_frames() / 60;

    //settings.walker.get_new_point(t as f64);
    settings.walker.get_new_point(model.tick);
    //model.tick += 0.1;
    model.tick += settings.noise_step;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let primary = rgb_u32(0x181D31);
    //draw.background().color(PLUM);
    draw.background().color(primary);

    if model.settings.show_grid {
        model.settings.walker.grid.draw(&draw);
    }

    model.settings.walker.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
