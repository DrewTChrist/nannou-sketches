use nannou::prelude::*;

pub struct FrameCapturer {
    pub texture: wgpu::Texture,
    pub draw: nannou::Draw,
    pub renderer: nannou::draw::Renderer,
    pub texture_capturer: wgpu::TextureCapturer,
    pub texture_reshaper: wgpu::TextureReshaper,
    frame_count: u32,
}

impl FrameCapturer {
    pub fn new(window: &Window, texture_size: [u32; 2]) -> Self {
        // Retrieve the wgpu device.
        let device = window.device();

        // Create our custom texture.
        let sample_count = window.msaa_samples();
        let texture = wgpu::TextureBuilder::new()
            .size(texture_size)
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
        Self {
            texture,
            draw,
            renderer,
            texture_capturer,
            texture_reshaper,
            frame_count: 0,
        }
    }
    //pub fn capture(&mut self, window: &Window) {
    pub fn capture(&mut self, app: &App) {
        let window = app.main_window();
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self.renderer
            .render_to_texture(device, &mut encoder, &self.draw, &self.texture);

        // Take a snapshot of the texture. The capturer will do the following:
        //
        // 1. Resolve the texture to a non-multisampled texture if necessary.
        // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
        // 3. Copy the result to a buffer ready to be mapped for reading.
        let snapshot = self
            .texture_capturer
            .capture(device, &mut encoder, &self.texture);

        // Submit the commands for our drawing and texture capture to the GPU.
        window.queue().submit(Some(encoder.finish()));

        // Submit a function for writing our snapshot to a PNG.
        //
        // NOTE: It is essential that the commands for capturing the snapshot are `submit`ted before we
        // attempt to read the snapshot - otherwise we will read a blank texture!
        let path = capture_directory(app)
            .join(self.frame_count.to_string())
            .with_extension("png");
        snapshot
            .read(move |result| {
                let image = result.expect("failed to map texture memory").to_owned();
                image
                    .save(&path)
                    .expect("failed to save texture to png image");
            })
            .unwrap();
        self.frame_count += 1;
    }
}

fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}
