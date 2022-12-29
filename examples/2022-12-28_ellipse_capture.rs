use nannou::color::Alpha;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use palette::blend::{Equations, Parameter};
use palette::{Blend, LinSrgba};

fn main() {
    //nannou::app(model).update(update).run();
    nannou::app(model).update(update).exit(exit).run();
}

const TEXTURE_SIZE: [u32; 2] = [3_840, 2_160];

struct Ellipse {
    x_y: Vec2,
    width: f32,
    height: f32,
    //color: Rgb<u8>,
    color: Alpha<Rgb<f32>, f32>,
    nstep: f32,
}

impl Ellipse {
    fn new(x_y: Vec2, width: f32, height: f32, color: Alpha<Rgb<f32>, f32>, nstep: f32) -> Self {
        Self {
            x_y,
            width,
            height,
            color,
            nstep,
        }
    }

    fn draw(&self, draw: &Draw, model: &Model) {
        let blend_mode =
            Equations::from_parameters(Parameter::SourceColor, Parameter::DestinationAlpha);
        let a = LinSrgba::new(
            self.color.color.red,
            self.color.color.green,
            self.color.color.blue,
            self.color.alpha,
        );
        let b = LinSrgba::new(
            model.fade.color.red,
            model.fade.color.green,
            model.fade.color.blue,
            model.fade.alpha,
        );
        let c = a.blend(b, blend_mode);
        draw.ellipse()
            //.color(self.color)
            .color(c)
            .w_h(self.width, self.height)
            .x_y(self.x_y.x, self.x_y.y);
    }
}

struct Model {
    ell: Vec<Ellipse>,
    noise: Perlin,
    fade: Alpha<Rgb<f32>, f32>,
    texture: wgpu::Texture,
    draw: nannou::Draw,
    renderer: nannou::draw::Renderer,
    texture_capturer: wgpu::TextureCapturer,
    texture_reshaper: wgpu::TextureReshaper,
}

fn model(app: &App) -> Model {
    let [win_w, win_h] = [TEXTURE_SIZE[0] / 4, TEXTURE_SIZE[1] / 4];
    let window_id = app
        .new_window()
        .size(win_w, win_h)
        .view(view)
        .build()
        .unwrap();
    //let window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let bounds = app.window_rect();

    let mut ell = Vec::<Ellipse>::new();
    let mut x = bounds.left();
    let mut gray = 0.1;
    let mut nstep = random_range(0.0, 500.0);
    while x <= bounds.right() {
        ell.push(Ellipse::new(
            pt2(x, 0.0),
            25.0,
            250.0,
            srgba(gray, gray, gray, 0.1),
            nstep,
        ));
        //x += 30.0;
        x += 20.0;
        //gray += 0.01;
        gray += 0.05;
        //nstep += 0.125;
        nstep += 0.075;
    }
    let window = app.window(window_id).unwrap();

    // Retrieve the wgpu device.
    let device = window.device();

    // Create our custom texture.
    let sample_count = window.msaa_samples();
    let texture = wgpu::TextureBuilder::new()
        .size(TEXTURE_SIZE)
        // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
        // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        // Use nannou's default multisampling sample count.
        .sample_count(sample_count)
        // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
        .format(wgpu::TextureFormat::Rgba16Float)
        // Build it!
        .build(device);

    // Create our `Draw` instance and a renderer for it.
    let draw = nannou::Draw::new();
    let descriptor = texture.descriptor();
    let renderer =
        nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

    // Create the texture capturer.
    let texture_capturer = wgpu::TextureCapturer::default();

    // Create the texture reshaper.
    let texture_view = texture.view().build();
    let texture_sample_type = texture.sample_type();
    let dst_format = Frame::TEXTURE_FORMAT;
    let texture_reshaper = wgpu::TextureReshaper::new(
        device,
        &texture_view,
        sample_count,
        texture_sample_type,
        sample_count,
        dst_format,
    );

    // Make sure the directory where we will save images to exists.
    std::fs::create_dir_all(&capture_directory(app)).unwrap();

    Model {
        ell,
        noise: Perlin::new(),
        //fade: srgba(0.0, 0.0, 0.0, 0.3)
        fade: srgba(0.09, 0.09, 0.09, 0.15),
        texture,
        draw,
        renderer,
        texture_capturer,
        texture_reshaper,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let draw = &model.draw;
    draw.reset();
    let elapsed_frames = app.main_window().elapsed_frames();
    let bounds = app.window_rect();
    for ell in model.ell.iter_mut() {
        let noise = map_range(
            model.noise.get([ell.nstep as f64, 0.0]),
            -1.0,
            1.0,
            0.0,
            bounds.top(),
        );
        let color_noise = map_range(
            model.noise.get([ell.nstep as f64 + 50.0, 0.0]),
            -1.0,
            1.0,
            0.0,
            1.0,
        );
        ell.width = noise;
        ell.height = noise * 2.0;
        ell.nstep += 0.0125;
        ell.color.red = color_noise;
        ell.color.blue = color_noise * 2.0;
    }
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        //.w_h(600.0, 600.0)
        .w_h(TEXTURE_SIZE[0] as f32, TEXTURE_SIZE[1] as f32)
        .color(model.fade);

    for ell in &model.ell {
        ell.draw(&draw, &model);
    }

    let window = app.main_window();
    let device = window.device();
    let ce_desc = wgpu::CommandEncoderDescriptor {
        label: Some("texture renderer"),
    };
    let mut encoder = device.create_command_encoder(&ce_desc);
    model
        .renderer
        .render_to_texture(device, &mut encoder, draw, &model.texture);

    // Take a snapshot of the texture. The capturer will do the following:
    //
    // 1. Resolve the texture to a non-multisampled texture if necessary.
    // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
    // 3. Copy the result to a buffer ready to be mapped for reading.
    let snapshot = model
        .texture_capturer
        .capture(device, &mut encoder, &model.texture);

    // Submit the commands for our drawing and texture capture to the GPU.
    window.queue().submit(Some(encoder.finish()));

    // Submit a function for writing our snapshot to a PNG.
    //
    // NOTE: It is essential that the commands for capturing the snapshot are `submit`ted before we
    // attempt to read the snapshot - otherwise we will read a blank texture!
    let path = capture_directory(app)
        .join(elapsed_frames.to_string())
        .with_extension("png");
    snapshot
        .read(move |result| {
            let image = result.expect("failed to map texture memory").to_owned();
            image
                .save(&path)
                .expect("failed to save texture to png image");
        })
        .unwrap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut encoder = frame.command_encoder();
    model
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model
        .texture_capturer
        .await_active_snapshots(&device)
        .unwrap();
    println!("Done!");
}

// The directory where we'll save the frames.
fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}
