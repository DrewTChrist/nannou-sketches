use nannou::noise::NoiseFn;
use nannou::noise::Perlin;
/// Circle of circles rotating
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(500, 500).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLACK);

    //let sin = app.time.sin() * 100.0;
    //let cos = app.time.cos() * 100.0;

    let perlin = Perlin::new();

    //let draw = draw.color_blend(BLEND_ADD);

    for i in 1..250 {
        let sin = ((app.time / 2.0) - i as f32).sin() * 150.0;
        let cos = ((app.time / 2.0) - i as f32).cos() * 150.0;

        draw.ellipse()
            .x_y(sin, cos)
            //.w_h(50.0 * (app.time/i as f32).sin(), 50.0 * app.time.cos())
            //.radius((app.time / 20.0).sin() * 25.0 * (i as f32 * 0.0025))
            //.radius((app.time / 20.0).sin() * 25.0 * (i as f32 * app.time.cos() * 0.1))
            .radius(
                (perlin.get([
                    (i as f64 / 10.0).sin() * (app.time as f64 / 2.0 as f64),
                    (i as f64 / 10.0).cos() * (app.time as f64 / 5.0 as f64),
                ]) as f32)
                    * 45.0,
            )
            //.color(PLUM);
            .rgb(200.0, 200.0, 200.0);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
