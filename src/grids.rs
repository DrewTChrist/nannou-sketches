use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

pub struct Cell {
    coords: Vec2,
    color: Rgb<f32>,
    width: f32,
    height: f32,
}

impl Cell {
    pub fn new(coords: Vec2, color: Rgb<f32>, width: f32, height: f32) -> Self {
        Self {
            coords,
            color,
            width,
            height,
        }
    }
    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .x_y(self.coords.x, self.coords.y)
            .w_h(self.width, self.height)
            .color(self.color);
    }
}

pub struct NoiseGrid {
    grid: Vec<Vec<Cell>>,
    noise: Perlin,
    noise_values: (f64, f64),
}

impl NoiseGrid {
    pub fn new(location: Vec2, rows: usize, cols: usize, cell_size: f32) -> Self {
        let mut grid = Vec::<Vec<Cell>>::new();
        for r in 0..rows {
            let mut row = Vec::<Cell>::new();
            for c in 0..cols {
                let col = Cell::new(
                    pt2(
                        (location.x + r as f32) + (r as f32 * cell_size),
                        (location.y + c as f32) + (c as f32 * cell_size),
                    ),
                    srgb(r as f32, c as f32, 255.0),
                    cell_size,
                    cell_size,
                );
                row.push(col);
            }
            grid.push(row);
        }
        Self {
            grid,
            noise: Perlin::new(),
            noise_values: (random_range(-500.0, 500.0), random_range(-500.0, 500.0)),
        }
    }
    pub fn update_colors(&mut self) {
        for row in self.grid.iter_mut() {
            for col in row.iter_mut() {
                let noise = self.noise.get([
                    col.coords.x as f64 * 0.01 + self.noise_values.0,
                    col.coords.y as f64 * 0.01 + self.noise_values.1,
                ]);
                let r = map_range(self.noise.get([noise, 0.0]), -1.0, 1.0, 0.0, 1.0);
                let g = map_range(self.noise.get([r, 0.0]), 0.0, 1.0, 0.0, 1.0);
                let b = map_range(self.noise.get([g, 0.0]), 0.0, 1.0, 0.0, 1.0);
                col.color = rgb(r as f32, g as f32, b as f32);
                self.noise_values.0 += 0.000001;
                self.noise_values.1 += 0.000001;
            }
        }
    }
    pub fn draw(&self, draw: &Draw) {
        for row in &self.grid {
            for col in row {
                col.draw(draw);
            }
        }
    }
}

pub fn hex_grid(grid: &mut Vec<Vec2>, radius: f32) {
    //let size = 15.0;
    let size = radius;
    let wid = 2.0 * size;
    let h = 3.0.sqrt() * size;
    for i in -20..20 {
        for j in -20..20 {
            let x1 = i as f32 * 6.0 * wid / 4.0;
            let y1 = j as f32 * h;
            let mut t = 0.0;
            for _ in 0..6 {
                let x = x1 + t.cos() * size;
                let y = y1 + t.sin() * size;
                grid.push(pt2(x, y));
                t += (2.0 * PI) / 6.0;
            }
            let x2 = i as f32 * 6.0 * wid / 4.0 + 3.0 * wid / 4.0;
            let y2 = j as f32 * h + h / 2.0;
            let mut t = 0.0;
            for _ in 0..6 {
                let x = x2 + t.cos() * size;
                let y = y2 + t.sin() * size;
                grid.push(pt2(x, y));
                t += (2.0 * PI) / 6.0;
            }
        }
    }
}
