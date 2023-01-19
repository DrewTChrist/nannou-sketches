use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let _win = app.window_rect();

    draw.background().color(CORNFLOWERBLUE);

    let mut tris = Vec::new();

    let max = 2.0 * PI;
    let radius = 250.0;
    let x = 0.0;
    let y = 0.0;
    let offset = 0.0;
    let outer_shade = 1.0;
    let outer_color = lin_srgba(outer_shade, outer_shade, outer_shade, 1.0);
    let inner_shade = 0.0;
    let inner_color = lin_srgba(inner_shade, inner_shade, inner_shade, 1.0);

    let mut i = 0.0;
    while i <= max {
        let theta = i * max;
        let p = pt3(offset + theta.sin(), offset + theta.cos(), 0.0) * radius;
        tris.push((pt3(x, y, 0.0), inner_color));
        tris.push((p, outer_color));
        i += 0.01;
    }

    draw.mesh().points_colored(tris);

    draw.to_frame(app, &frame).unwrap();
}
