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

    let mut i = 0.0;
    while i <= max {
        let theta = i * max;
        let p = pt3(10.0 + theta.sin(), 10.0 + theta.cos(), 0.0) * 25.0;
        tris.push((pt3(250.0, 250.0, 0.0), lin_srgba(1.0, 1.0, 1.0, 1.0)));
        tris.push((p, lin_srgba(0.0, 0.0, 0.0, 1.0)));
        i += 0.01;
    }

    draw.mesh().points_colored(tris);

    draw.to_frame(app, &frame).unwrap();
}
