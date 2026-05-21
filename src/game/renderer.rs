use crate::game::snake::{CellType, SnakeGame};
use crate::gpu::{Buffer, GpuContext, RenderPipeline, RenderPipelineBuilder, ShaderModule};
use bytemuck::{Pod, Zeroable};

const SHADER_SRC: &str = r#"
struct Uniforms {
    grid_size: vec2<f32>,
    screen_size: vec2<f32>,
    aspect: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> cells: array<u32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) cell_type: f32,
};

fn cell_rect(cell_idx: u32) -> vec4<f32> {
    let grid_x = uniforms.grid_size.x;
    let grid_y = uniforms.grid_size.y;

    let col = i32(cell_idx) % i32(grid_x);
    let row = i32(cell_idx) / i32(grid_x);

    let cell_w = 1.0 / grid_x;
    let cell_h = 1.0 / grid_y;

    let padding = 0.04;
    let x = f32(col) * cell_w * 2.0 - 1.0 + cell_w + cell_w * padding;
    let y = 1.0 - f32(row) * cell_h * 2.0 - cell_h + cell_h * padding;

    let w = cell_w * (1.0 - padding * 2.0);
    let h = cell_h * (1.0 - padding * 2.0);

    return vec4<f32>(x, y, w, h);
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let cell_idx = vertex_index / 6u;
    let cell_type = cells[cell_idx];

    let rect = cell_rect(cell_idx);
    let x = rect.x;
    let y = rect.y;
    let w = rect.z;
    let h = rect.w;

    var positions = array<vec2<f32>, 6>(
        vec2<f32>(x, y),
        vec2<f32>(x + w, y),
        vec2<f32>(x, y + h),
        vec2<f32>(x + w, y),
        vec2<f32>(x, y + h),
        vec2<f32>(x + w, y + h),
    );

    var out: VertexOutput;
    out.position = vec4<f32>(positions[vertex_index % 6u], 0.0, 1.0);
    out.cell_type = f32(cell_type);
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let t = input.cell_type;

    let bg_color = vec4<f32>(0.08, 0.08, 0.15, 1.0);
    let grid_color = vec4<f32>(0.12, 0.12, 0.22, 1.0);
    let head_color = vec4<f32>(0.2, 1.0, 0.3, 1.0);
    let body_color = vec4<f32>(0.1, 0.75, 0.2, 1.0);
    let tail_color = vec4<f32>(0.05, 0.55, 0.12, 1.0);
    let food_color = vec4<f32>(1.0, 0.25, 0.25, 1.0);

    var color = bg_color;

    if (t > 0.5 && t < 1.5) {
        color = head_color;
    } else if (t > 1.5 && t < 2.5) {
        color = body_color;
    } else if (t > 2.5 && t < 3.5) {
        color = tail_color;
    } else if (t > 3.5 && t < 4.5) {
        color = food_color;
    } else if (t > 4.5 && t < 5.5) {
        color = grid_color;
    }

    return color;
}
"#;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
    grid_size: [f32; 2],
    screen_size: [f32; 2],
    aspect: f32,
    _pad: f32,
}

pub struct SnakeRenderer {
    pipeline: RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: Buffer,
    cell_buffer: Buffer,
    cell_count: u32,
}

impl SnakeRenderer {
    pub fn new(gpu: &GpuContext, game: &SnakeGame) -> Self {
        let shader = ShaderModule::from_wgsl(&gpu.device, "Snake Shader", SHADER_SRC);

        let uniform_buffer = Buffer::new_uniform(
            &gpu.device,
            "Uniform Buffer",
            bytemuck::bytes_of(&Uniforms {
                grid_size: [game.grid_width as f32, game.grid_height as f32],
                screen_size: [gpu.surface_config.width as f32, gpu.surface_config.height as f32],
                aspect: game.grid_width as f32 / game.grid_height as f32,
                _pad: 0.0,
            }),
        );

        let total_cells = (game.grid_width * game.grid_height) as u32;
        let cell_data: Vec<u32> = vec![0; total_cells as usize];

        let cell_buffer = Buffer::new_storage(
            &gpu.device,
            "Cell Buffer",
            (total_cells * 4) as u64,
        );
        cell_buffer.write_data(&gpu.queue, bytemuck::cast_slice(&cell_data));

        let bind_group_layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Snake Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Snake Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.inner.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: cell_buffer.inner.as_entire_binding(),
                },
            ],
        });

        let pipeline = RenderPipelineBuilder::new("Snake Pipeline", gpu.surface_format())
            .shader(shader)
            .topology(wgpu::PrimitiveTopology::TriangleList)
            .build(&gpu.device, &[&bind_group_layout]);

        Self {
            pipeline,
            bind_group,
            uniform_buffer,
            cell_buffer,
            cell_count: total_cells,
        }
    }

    pub fn update(&self, gpu: &GpuContext, game: &SnakeGame) {
        let uniforms = Uniforms {
            grid_size: [game.grid_width as f32, game.grid_height as f32],
            screen_size: [gpu.surface_config.width as f32, gpu.surface_config.height as f32],
            aspect: game.grid_width as f32 / game.grid_height as f32,
            _pad: 0.0,
        };
        self.uniform_buffer
            .write_data(&gpu.queue, bytemuck::bytes_of(&uniforms));

        let total_cells = (game.grid_width * game.grid_height) as usize;
        let mut cell_data: Vec<u32> = vec![0; total_cells];

        for y in 0..game.grid_height {
            for x in 0..game.grid_width {
                let idx = (y * game.grid_width + x) as usize;
                let cell_type = match game.get_cell(x, y) {
                    CellType::Empty => {
                        if x == 0 || y == 0 || x == game.grid_width - 1 || y == game.grid_height - 1 {
                            5
                        } else {
                            0
                        }
                    }
                    CellType::Head => 1,
                    CellType::Body => 2,
                    CellType::Tail => 3,
                    CellType::Food => 4,
                };
                cell_data[idx] = cell_type;
            }
        }

        self.cell_buffer
            .write_data(&gpu.queue, bytemuck::cast_slice(&cell_data));
    }

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Snake Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline.inner);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..self.cell_count * 6, 0..1);
    }
}
