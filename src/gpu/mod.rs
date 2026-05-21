use std::sync::Arc;
use winit::window::Window;

mod buffer;
mod pipeline;
mod texture;
mod text;
mod geom;

pub use buffer::*;
pub use pipeline::*;
pub use texture::*;
pub use text::*;
pub use geom::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub text_manager: std::sync::Mutex<TextManager>,
}

impl GpuContext {
    pub async fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
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
                    required_limits: wgpu::Limits::default(),
                    ..Default::default()
                },
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

        let text_manager = TextManager::new(&device, &queue, &surface_format);

        Self {
            device,
            queue,
            surface,
            surface_config,
            text_manager: std::sync::Mutex::new(text_manager),
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
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => Some(texture),
            _ => None,
        }
    }
}

pub struct Canvas {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,
    max_vertices: usize,
    text_queue: Vec<(cosmic_text::Buffer, f32, f32, [f32; 4])>,
    transform_stack: Vec<Mat3>,
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
            immediate_size: 0,
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
            multiview_mask: None,
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
            text_queue: Vec::new(),
            transform_stack: vec![Mat3::identity()],
        }
    }

    pub fn push_transform(&mut self, transform: Mat3) {
        let current = self.current_transform();
        self.transform_stack.push(current.multiply(&transform));
    }

    pub fn pop_transform(&mut self) {
        if self.transform_stack.len() > 1 {
            self.transform_stack.pop();
        }
    }

    fn current_transform(&self) -> Mat3 {
        *self.transform_stack.last().unwrap_or(&Mat3::identity())
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        if self.vertices.len() + 6 > self.max_vertices {
            return;
        }
        
        let transform = self.current_transform();
        
        let p1 = transform.transform_point(Vec2::new(x, y));
        let p2 = transform.transform_point(Vec2::new(x + w, y));
        let p3 = transform.transform_point(Vec2::new(x, y + h));
        let p4 = transform.transform_point(Vec2::new(x + w, y + h));

        let v1 = Vertex { position: [p1.x, p1.y], color };
        let v2 = Vertex { position: [p2.x, p2.y], color };
        let v3 = Vertex { position: [p3.x, p3.y], color };
        let v4 = Vertex { position: [p4.x, p4.y], color };

        self.vertices.extend_from_slice(&[v1, v2, v3, v2, v4, v3]);
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, r: f32, color: [f32; 4]) {
        if self.vertices.len() + (32 * 3) > self.max_vertices {
            return;
        }
        let transform = self.current_transform();
        let segments = 32;
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
            
            let p1 = transform.transform_point(Vec2::new(cx, cy));
            let p2 = transform.transform_point(Vec2::new(cx + angle1.cos() * r, cy + angle1.sin() * r));
            let p3 = transform.transform_point(Vec2::new(cx + angle2.cos() * r, cy + angle2.sin() * r));
            
            let v1 = Vertex { position: [p1.x, p1.y], color };
            let v2 = Vertex { position: [p2.x, p2.y], color };
            let v3 = Vertex { position: [p3.x, p3.y], color };
            
            self.vertices.extend_from_slice(&[v1, v2, v3]);
        }
    }

    pub fn draw_text(&mut self, gpu: &GpuContext, text: &str, x: f32, y: f32, font_size: f32, color: [f32; 4]) {
        let transform = self.current_transform();
        let pos = transform.transform_point(Vec2::new(x, y));
        
        let buffer = gpu.text_manager.lock().unwrap().create_buffer(text, font_size);
        self.text_queue.push((buffer, pos.x, pos.y, color));
    }

    pub fn end_drawing(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear_color: [f32; 4]) {
        gpu.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));

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
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.vertices.len() as u32, 0..1);
        }

        if !self.text_queue.is_empty() {
            let sw = gpu.surface_config.width as f32;
            let sh = gpu.surface_config.height as f32;
            
            let texts = std::mem::take(&mut self.text_queue);
            gpu.text_manager.lock().unwrap().render(&gpu.device, &gpu.queue, encoder, view, &texts, sw, sh);
        }

        self.vertices.clear();
        self.text_queue.clear();
    }
}
