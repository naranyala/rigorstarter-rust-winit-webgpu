use crate::gpu::GpuContext;
use crate::stdlib::{State, StateRequest, InputManager};
use crate::stdlib::linear_algebra::{Mat4, Vec3};
use winit::keyboard::KeyCode;
use winit::event::ElementState;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
}

const VERTICES: &[Vertex] = &[
    // Front face
    Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 1.0, 1.0] },
    // Back face
    Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 0.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 0.0, 1.0] },
    // Left face
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.5, 0.0, 0.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.5, 0.5, 0.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.5, 0.0, 1.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 0.0, 0.5, 1.0] },
    // Right face
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.5, 0.0, 0.5, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.5, 0.5, 0.5, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.5, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.5, 1.0] },
    // Top face
    Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 1.0, 1.0] },
    // Bottom face
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0] },
];

const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,       // Front
    4, 5, 6, 6, 7, 4,       // Back
    8, 9, 10, 10, 11, 8,    // Left
    12, 13, 14, 14, 15, 12, // Right
    16, 17, 18, 18, 19, 16, // Top
    20, 21, 22, 22, 23, 20, // Bottom
];

const SHADER_SOURCE: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Uniforms {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = uniforms.mvp * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

pub struct CubeDemoState {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    rotation_angle: f32,
}

impl CubeDemoState {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = &gpu.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cube Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cube Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4],
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
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (VERTICES.len() * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        gpu.queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(VERTICES));

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: (INDICES.len() * std::mem::size_of::<u16>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        gpu.queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(INDICES));

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group,
            rotation_angle: 0.0,
        }
    }
}

impl State for CubeDemoState {
    fn update(&mut self, delta: f32, _input: &InputManager) -> Option<StateRequest> {
        self.rotation_angle += delta * 1.0;
        None
    }

    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let width = gpu.surface_config.width as f32;
        let height = gpu.surface_config.height as f32;

        // Calculate MVP
        let projection = Mat4::perspective(std::f32::consts::FRAC_PI_4, width / height, 0.1, 100.0);
        let view_mat = Mat4::look_at(Vec3::new(0.0, 1.5, 3.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let model = Mat4::rotation_x(self.rotation_angle) * Mat4::rotation_y(self.rotation_angle * 0.5);
        let mvp = projection * view_mat * model;

        let uniforms = Uniforms {
            mvp: [
                [mvp.data[0], mvp.data[1], mvp.data[2], mvp.data[3]],
                [mvp.data[4], mvp.data[5], mvp.data[6], mvp.data[7]],
                [mvp.data[8], mvp.data[9], mvp.data[10], mvp.data[11]],
                [mvp.data[12], mvp.data[13], mvp.data[14], mvp.data[15]],
            ],
        };

        gpu.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        if state == ElementState::Pressed && key == KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }
}
