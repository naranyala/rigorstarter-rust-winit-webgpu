use glyphon::{
    FontSystem, TextAtlas, TextRenderer,
    Cache, Viewport, TextArea
};
use cosmic_text::{Buffer, Metrics, Attrs, Shaping, Align, SwashCache};
use wgpu::{Device, Queue, TextureFormat};

pub struct TextManager {
    pub font_system: FontSystem,
    pub atlas: TextAtlas,
    pub renderer: TextRenderer,
    pub cache: Cache,
    pub viewport: Viewport,
    pub swash_cache: SwashCache,
}

impl TextManager {
    pub fn new(device: &Device, queue: &Queue, format: &TextureFormat) -> Self {
        let font_system = FontSystem::new();
        let cache = Cache::new(device);
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, &cache, *format);
        let renderer = TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);
        let viewport = Viewport::new(device, &cache);

        Self {
            font_system,
            atlas,
            renderer,
            cache,
            viewport,
            swash_cache,
        }
    }

    pub fn create_buffer(&mut self, text: &str, size: f32) -> Buffer {
        let metrics = Metrics::new(size, size);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(
            &mut self.font_system,
            Some(f32::INFINITY),
            Some(f32::INFINITY),
        );
        buffer.set_text(
            &mut self.font_system,
            text,
            &Attrs::new(),
            Shaping::Advanced,
            Some(Align::Left),
        );
        buffer
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        buffers: &[(Buffer, f32, f32, [f32; 4])],
        screen_width: f32,
        screen_height: f32,
    ) {
        self.viewport.update(
            queue,
            glyphon::Resolution {
                width: screen_width as u32,
                height: screen_height as u32,
            },
        );

        let text_areas = buffers.iter().map(|(buffer, x, y, color)| {
            TextArea {
                buffer,
                left: *x,
                top: *y,
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: 0,
                    top: 0,
                    right: screen_width as i32,
                    bottom: screen_height as i32,
                },
                default_color: glyphon::Color::rgb(
                    (color[0] * 255.0) as u8,
                    (color[1] * 255.0) as u8,
                    (color[2] * 255.0) as u8,
                ),
                custom_glyphs: &[],
            }
        });

        self.renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            text_areas,
            &mut self.swash_cache,
        ).expect("Failed to prepare text");

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Text Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        self.renderer.render(
            &self.atlas,
            &self.viewport,
            &mut render_pass,
        ).expect("Failed to render text");
    }
}
