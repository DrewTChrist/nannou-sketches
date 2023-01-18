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

const HEIGHT: isize = 300;
const WIDTH: isize = 300;
const STEP: usize = 10;

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
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

struct Walker {
    start: Vec2,
    points: Vec<Vec2>,
    grid: Grid,
    current_row: i32,
    current_col: i32,
    time_step: u64,
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
            time_step: 0,
        }
    }
    fn draw(&self, draw: &Draw) {
        for i in 1..self.points.len() {
            draw.line()
                .start(self.points[i - 1] - 1.0)
                .end(self.points[i] - 1.0)
                .color(GRAY)
                .weight(2.0);
            draw.line()
                .start(self.points[i - 1])
                .end(self.points[i])
                .color(BLACK)
                .weight(2.0);
        }
    }
    /* - Make sure next point isn't already in the path
     * - Make sure it doesn't go outside of the grid boundaries */
    fn get_new_point(&mut self) {
        let mut rand_direction = DIRECTIONS[random_range(0, 8)];
        let mut new_location = rand_direction.to_row_col(self.current_row, self.current_col);
        while new_location.0 < 0
            || new_location.0 > (HEIGHT as i32 * 2 / STEP as i32) - 1
            || new_location.1 < 0
            || new_location.1 > (WIDTH as i32 * 2 / STEP as i32) - 1
        {
            rand_direction = DIRECTIONS[random_range(0, 8)];
            new_location = rand_direction.to_row_col(self.current_row, self.current_col);
        }
        let next_point = self.grid.points[new_location.0 as usize][new_location.1 as usize];
        if !self.points.contains(&next_point) {
            self.points.push(next_point);
            self.current_row = new_location.0;
            self.current_col = new_location.1;
        }
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

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let mut reset: bool = false;
    let mut show_grid: bool = false;

    egui::Window::new("Settings").show(&ctx, |ui| {
        show_grid = ui.button("Show grid").clicked();

        reset = ui.button("Reset").clicked();
    });

    if show_grid {
        settings.show_grid = !settings.show_grid;
    }

    if reset {
        *settings = Settings::default();
    }

    model.settings.walker.get_new_point();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    if model.settings.show_grid {
        model.settings.walker.grid.draw(&draw);
    }

    model.settings.walker.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
