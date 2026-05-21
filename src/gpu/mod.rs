use std::sync::Arc;
use winit::window::Window;

mod buffer;
mod pipeline;
mod texture;

pub use buffer::*;
pub use pipeline::*;
pub use texture::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

// 5x7 Bitmap Font Data
pub const FONT_DATA: &[ [u8; 7] ] = &[
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // space
    [0x00, 0x00, 0x5F, 0x00, 0x00, 0x00, 0x00], // !
    [0x00, 0x07, 0x00, 0x07, 0x00, 0x00, 0x00], // "
    [0x14, 0x7F, 0x14, 0x7F, 0x14, 0x00, 0x00], // #
    [0x24, 0x2A, 0x7F, 0x2A, 0x12, 0x00, 0x00], // $
    [0x23, 0x13, 0x08, 0x64, 0x62, 0x00, 0x00], // %
    [0x36, 0x49, 0x55, 0x22, 0x50, 0x00, 0x00], // &
    [0x00, 0x05, 0x03, 0x00, 0x00, 0x00, 0x00], // '
    [0x00, 0x1C, 0x22, 0x41, 0x00, 0x00, 0x00], // (
    [0x00, 0x41, 0x22, 0x1C, 0x00, 0x00, 0x00], // )
    [0x14, 0x08, 0x3E, 0x08, 0x14, 0x00, 0x00], // *
    [0x08, 0x08, 0x3E, 0x08, 0x08, 0x00, 0x00], // +
    [0x00, 0x50, 0x30, 0x00, 0x00, 0x00, 0x00], // ,
    [0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00], // -
    [0x00, 0x60, 0x60, 0x00, 0x00, 0x00, 0x00], // .
    [0x20, 0x10, 0x08, 0x04, 0x02, 0x00, 0x00], // /
    [0x3E, 0x51, 0x49, 0x45, 0x3E, 0x00, 0x00], // 0
    [0x00, 0x42, 0x7F, 0x40, 0x00, 0x00, 0x00], // 1
    [0x42, 0x61, 0x51, 0x49, 0x46, 0x00, 0x00], // 2
    [0x21, 0x41, 0x45, 0x4B, 0x31, 0x00, 0x00], // 3
    [0x18, 0x14, 0x12, 0x7F, 0x10, 0x00, 0x00], // 4
    [0x27, 0x45, 0x45, 0x45, 0x39, 0x00, 0x00], // 5
    [0x3C, 0x4A, 0x49, 0x49, 0x30, 0x00, 0x00], // 6
    [0x01, 0x71, 0x09, 0x05, 0x03, 0x00, 0x00], // 7
    [0x36, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00], // 8
    [0x06, 0x49, 0x49, 0x29, 0x1E, 0x00, 0x00], // 9
    [0x00, 0x36, 0x36, 0x00, 0x00, 0x00, 0x00], // :
    [0x00, 0x56, 0x36, 0x00, 0x00, 0x00, 0x00], // ;
    [0x08, 0x14, 0x22, 0x41, 0x00, 0x00, 0x00], // <
    [0x14, 0x14, 0x14, 0x14, 0x14, 0x00, 0x00], // =
    [0x00, 0x41, 0x22, 0x14, 0x08, 0x00, 0x00], // >
    [0x02, 0x01, 0x51, 0x09, 0x06, 0x00, 0x00], // ?
    [0x32, 0x49, 0x79, 0x41, 0x3E, 0x00, 0x00], // @
    [0x7E, 0x11, 0x11, 0x11, 0x7E, 0x00, 0x00], // A
    [0x7F, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00], // B
    [0x3E, 0x41, 0x41, 0x41, 0x22, 0x00, 0x00], // C
    [0x7F, 0x41, 0x41, 0x22, 0x1C, 0x00, 0x00], // D
    [0x7F, 0x49, 0x49, 0x49, 0x41, 0x00, 0x00], // E
    [0x7F, 0x09, 0x09, 0x09, 0x01, 0x00, 0x00], // F
    [0x3E, 0x41, 0x49, 0x49, 0x7A, 0x00, 0x00], // G
    [0x7F, 0x08, 0x08, 0x08, 0x7F, 0x00, 0x00], // H
    [0x00, 0x41, 0x7F, 0x41, 0x00, 0x00, 0x00], // I
    [0x20, 0x40, 0x41, 0x3F, 0x01, 0x00, 0x00], // J
    [0x7F, 0x08, 0x14, 0x22, 0x41, 0x00, 0x00], // K
    [0x7F, 0x40, 0x40, 0x40, 0x40, 0x00, 0x00], // L
    [0x7F, 0x02, 0x0C, 0x02, 0x7F, 0x00, 0x00], // M
    [0x7F, 0x04, 0x08, 0x10, 0x7F, 0x00, 0x00], // N
    [0x3E, 0x41, 0x41, 0x41, 0x3E, 0x00, 0x00], // O
    [0x7F, 0x09, 0x09, 0x09, 0x06, 0x00, 0x00], // P
    [0x3E, 0x41, 0x51, 0x21, 0x5E, 0x00, 0x00], // Q
    [0x7F, 0x09, 0x19, 0x29, 0x46, 0x00, 0x00], // R
    [0x46, 0x49, 0x49, 0x49, 0x31, 0x00, 0x00], // S
    [0x01, 0x01, 0x7F, 0x01, 0x01, 0x00, 0x00], // T
    [0x3F, 0x40, 0x40, 0x40, 0x3F, 0x00, 0x00], // U
    [0x1F, 0x20, 0x40, 0x20, 0x1F, 0x00, 0x00], // V
    [0x3F, 0x40, 0x38, 0x40, 0x3F, 0x00, 0x00], // W
    [0x63, 0x14, 0x08, 0x14, 0x63, 0x00, 0x00], // X
    [0x07, 0x08, 0x70, 0x08, 0x07, 0x00, 0x00], // Y
    [0x61, 0x51, 0x49, 0x45, 0x43, 0x00, 0x00], // Z
    [0x00, 0x7F, 0x41, 0x41, 0x00, 0x00, 0x00], // [
    [0x02, 0x04, 0x08, 0x10, 0x20, 0x00, 0x00], // \
    [0x00, 0x41, 0x41, 0x7F, 0x00, 0x00, 0x00], // ]
    [0x04, 0x02, 0x01, 0x02, 0x04, 0x00, 0x00], // ^
    [0x40, 0x40, 0x40, 0x40, 0x40, 0x00, 0x00], // _
    [0x00, 0x01, 0x02, 0x04, 0x00, 0x00, 0x00], // `
    [0x20, 0x54, 0x54, 0x54, 0x78, 0x00, 0x00], // a
    [0x7F, 0x48, 0x44, 0x44, 0x38, 0x00, 0x00], // b
    [0x38, 0x44, 0x44, 0x44, 0x20, 0x00, 0x00], // c
    [0x38, 0x44, 0x44, 0x48, 0x7F, 0x00, 0x00], // d
    [0x38, 0x54, 0x54, 0x54, 0x18, 0x00, 0x00], // e
    [0x08, 0x7E, 0x09, 0x01, 0x02, 0x00, 0x00], // f
    [0x0C, 0x52, 0x52, 0x52, 0x3E, 0x00, 0x00], // g
    [0x7F, 0x08, 0x04, 0x04, 0x78, 0x00, 0x00], // h
    [0x00, 0x44, 0x7D, 0x40, 0x00, 0x00, 0x00], // i
    [0x20, 0x40, 0x44, 0x3D, 0x00, 0x00, 0x00], // j
    [0x7F, 0x10, 0x28, 0x44, 0x00, 0x00, 0x00], // k
    [0x00, 0x41, 0x7F, 0x40, 0x00, 0x00, 0x00], // l
    [0x7C, 0x04, 0x18, 0x04, 0x78, 0x00, 0x00], // m
    [0x7C, 0x08, 0x04, 0x04, 0x78, 0x00, 0x00], // n
    [0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0x00], // o
    [0x7C, 0x14, 0x14, 0x14, 0x08, 0x00, 0x00], // p
    [0x08, 0x14, 0x14, 0x18, 0x7C, 0x00, 0x00], // q
    [0x7C, 0x08, 0x04, 0x04, 0x08, 0x00, 0x00], // r
    [0x48, 0x54, 0x54, 0x54, 0x20, 0x00, 0x00], // s
    [0x04, 0x3F, 0x44, 0x40, 0x20, 0x00, 0x00], // t
    [0x3C, 0x40, 0x40, 0x20, 0x7C, 0x00, 0x00], // u
    [0x1C, 0x20, 0x40, 20, 0x1C, 0x00, 0x00], // v
    [0x3C, 0x40, 0x30, 0x40, 0x3C, 0x00, 0x00], // w
    [0x44, 0x28, 0x10, 0x28, 0x44, 0x00, 0x00], // x
    [0x0C, 0x50, 0x50, 0x50, 0x3C, 0x00, 0x00], // y
    [0x44, 0x64, 0x54, 0x4C, 0x44, 0x00, 0x00], // z
    [0x00, 0x08, 0x36, 0x41, 0x00, 0x00, 0x00], // {
    [0x00, 0x00, 0x7F, 0x00, 0x00, 0x00, 0x00], // |
    [0x00, 0x41, 0x36, 0x08, 0x00, 0x00, 0x00], // }
    [0x08, 0x04, 0x08, 0x10, 0x08, 0x00, 0x00], // ~
];

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    pub async fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    ..Default::default()
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        Self {
            device,
            queue,
            surface,
            surface_config,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }

    pub fn get_frame(&self) -> Option<wgpu::SurfaceTexture> {
        self.surface.get_current_texture().ok()
    }
}

pub struct Canvas {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,
    max_vertices: usize,
}

impl Canvas {
    pub fn new(gpu: &GpuContext) -> Self {
        let shader = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Canvas Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("canvas.wgsl").into()),
        });

        let layout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Canvas Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Canvas Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: gpu.surface_format(),
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let max_vertices = 100_000;
        let vertex_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Canvas Vertex Buffer"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertices: Vec::with_capacity(max_vertices),
            max_vertices,
        }
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        if self.vertices.len() + 6 > self.max_vertices {
            return;
        }
        let v1 = Vertex { position: [x, y], color };
        let v2 = Vertex { position: [x + w, y], color };
        let v3 = Vertex { position: [x, y + h], color };
        let v4 = Vertex { position: [x + w, y + h], color };

        self.vertices.extend_from_slice(&[v1, v2, v3, v2, v4, v3]);
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, r: f32, color: [f32; 4]) {
        let segments = 32;
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
            
            let v1 = Vertex { position: [cx, cy], color };
            let v2 = Vertex { position: [cx + angle1.cos() * r, cy + angle1.sin() * r], color };
            let v3 = Vertex { position: [cx + angle2.cos() * r, cy + angle2.sin() * r], color };
            
            self.vertices.extend_from_slice(&[v1, v2, v3]);
        }
    }

    pub fn draw_text(&mut self, text: &str, x: f32, y: f32, font_size: f32, color: [f32; 4]) {
        let mut cx = x;
        let pixel_w = font_size * 0.6;
        let pixel_h = font_size * 0.8;

        for c in text.chars() {
            if let Some(bits) = FONT_DATA.get((c as usize).saturating_sub(32)) {
                for row in 0..7 {
                    let row_bits = bits[row];
                    for col in 0..5 {
                        if (row_bits & (1 << (4 - col))) != 0 {
                            let px = cx + col as f32 * pixel_w;
                            let py = y + row as f32 * pixel_h;
                            self.draw_rectangle(px, py, pixel_w, pixel_h, color);
                        }
                    }
                }
                cx += 6.0 * pixel_w;
            }
        }
    }

    pub fn end_drawing(&mut self, gpu: &GpuContext, view: &wgpu::TextureView, clear_color: [f32; 4]) {
        gpu.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));

        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Canvas Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Canvas Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: clear_color[0] as f64,
                            g: clear_color[1] as f64,
                            b: clear_color[2] as f64,
                            a: clear_color[3] as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.vertices.len() as u32, 0..1);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        self.vertices.clear();
    }
}
