//! Squares in a grid getting smaller each step and rotating
//! into the center of the screen

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .size(500, 500)
        .update(update)
        .simple_window(view)
        .run();
}

const NUM_SQUARES: u32 = 100;
const SPEED: f32 = 1.0;

fn create_path(boundary: Rect<f32>) -> Vec<Vec2> {
    let mut path: Vec<Vec2> = Vec::<Vec2>::new();
    let _direction = Direction::Right;
    path.push(pt2(boundary.left(), boundary.top() - 100.0));
    //path.push(pt2(boundary.left() + 100.0, boundary.top() - 100.0));
    path.push(pt2(boundary.right() - 100.0, boundary.top() - 100.0));
    path.push(pt2(boundary.right() - 100.0, boundary.bottom() + 100.0));
    path.push(pt2(boundary.left() + 150.0, boundary.bottom() + 100.0));
    path.push(pt2(boundary.left() + 150.0, boundary.top() - 150.0));
    path
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Square {
    coords: Vec2,
    size: f32,
    direction: Direction,
    next: usize,
}

impl Square {
    fn new(coords: Vec2, size: f32) -> Self {
        Self {
            coords,
            size,
            direction: Direction::Right,
            next: 1,
        }
    }
    fn draw(&self, draw: &Draw) {
        draw.rect()
            .color(STEELBLUE)
            .w_h(self.size, self.size)
            .x_y(self.coords.x, self.coords.y);
    }
    fn change_direction(&mut self) {
        match self.direction {
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Up => self.direction = Direction::Right,
        }
    }
}

struct Model {
    squares: Vec<Square>,
    path: Vec<Vec2>,
}

fn model(app: &App) -> Model {
    let mut model = Model {
        squares: Vec::<Square>::new(),
        path: create_path(app.window_rect()),
    };
    //let mut size: f32 = 100.0;
    let mut size: f32 = 0.0;
    for i in 0..NUM_SQUARES {
        model.squares.push(Square::new(
            pt2(model.path[0].x - (i as f32 * 75.0), model.path[0].y),
            size,
        ));
        size += 5.0;
    }
    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for square in model.squares.iter_mut() {
        for i in 1..model.path.len() - 1 {
            if square.coords == model.path[i] {
                square.next = i + 1;
                square.change_direction();
            }
        }
        if square.coords != model.path[square.next] {
            match square.direction {
                Direction::Up => square.coords.y += SPEED,
                Direction::Down => square.coords.y -= SPEED,
                Direction::Left => square.coords.x -= SPEED,
                Direction::Right => square.coords.x += SPEED,
            }
        }
        if square.size > 0.0 {
            square.size -= 0.025;
        } /*else {
              // find the square that is furthest to
              // the left and then reset this square to
              // be behind that one
              square.coords.x = model.path[0].x - (idx as f32 * 75.0);
              square.coords.y = model.path[0].y;
              square.size = 75.0;
          }*/
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    for square in model.squares.iter() {
        square.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
