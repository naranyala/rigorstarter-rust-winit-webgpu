use std::collections::VecDeque;
use crate::gpu::{Buffer, GpuContext, RenderPipeline, RenderPipelineBuilder, ShaderModule, Vec2};
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Head,
    Body,
    Tail,
    Food,
}

pub struct SnakeGame {
    pub grid_width: i32,
    pub grid_height: i32,
    snake: VecDeque<Position>,
    direction: Direction,
    next_direction: Direction,
    food: Position,
    pub score: u32,
    pub game_over: bool,
    tick_accumulator: f64,
    pub tick_rate: f64,
}

impl SnakeGame {
    pub fn new(grid_width: i32, grid_height: i32) -> Self {
        let start = Position::new(grid_width / 2, grid_height / 2);
        let mut snake = VecDeque::new();
        snake.push_back(start);
        snake.push_back(Position::new(start.x - 1, start.y));
        snake.push_back(Position::new(start.x - 2, start.y));

        let mut game = Self {
            grid_width,
            grid_height,
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Position::new(0, 0),
            score: 0,
            game_over: false,
            tick_accumulator: 0.0,
            tick_rate: 0.3,
        };

        game.spawn_food();
        game
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if dir != self.direction.opposite() {
            self.next_direction = dir;
        }
    }

    fn spawn_food(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        loop {
            let pos = Position::new(
                rng.gen_range(0..self.grid_width),
                rng.gen_range(0..self.grid_height),
            );
            if !self.snake.iter().any(|s| s.x == pos.x && s.y == pos.y) {
                self.food = pos;
                break;
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.game_over {
            return;
        }

        self.tick_accumulator += delta_time;

        while self.tick_accumulator >= self.tick_rate {
            self.tick_accumulator -= self.tick_rate;
            self.tick();
        }
    }

    fn tick(&mut self) {
        self.direction = self.next_direction;

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position::new(head.x, head.y - 1),
            Direction::Down => Position::new(head.x, head.y + 1),
            Direction::Left => Position::new(head.x - 1, head.y),
            Direction::Right => Position::new(head.x + 1, head.y),
        };

        if new_head.x < 0
            || new_head.x >= self.grid_width
            || new_head.y < 0
            || new_head.y >= self.grid_height
        {
            self.game_over = true;
            return;
        }

        if self.snake.iter().any(|s| s.x == new_head.x && s.y == new_head.y) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 10;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> CellType {
        if self.snake.is_empty() {
            return CellType::Empty;
        }

        let head = self.snake.front().unwrap();
        let tail = self.snake.back().unwrap();

        if x == self.food.x && y == self.food.y {
            return CellType::Food;
        }

        if x == head.x && y == head.y {
            return CellType::Head;
        }

        if x == tail.x && y == tail.y {
            return CellType::Tail;
        }

        if self.snake.iter().any(|s| s.x == x && s.y == y) {
            return CellType::Body;
        }

        CellType::Empty
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.grid_width, self.grid_height);
    }
}

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

    let x = f32(col) * cell_w * 2.0 - 1.0;
    let y = 1.0 - f32(row) * cell_h * 2.0;

    let w = cell_w * 2.0;
    let h = cell_h * 2.0;

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
        vec2<f32>(x, y - h),
        vec2<f32>(x + w, y),
        vec2<f32>(x, y - h),
        vec2<f32>(x + w, y - h),
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
    grid_size: Vec2,
    screen_size: Vec2,
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
                grid_size: Vec2::new(game.grid_width as f32, game.grid_height as f32),
                screen_size: Vec2::new(gpu.surface_config.width as f32, gpu.surface_config.height as f32),
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
            grid_size: Vec2::new(game.grid_width as f32, game.grid_height as f32),
            screen_size: Vec2::new(gpu.surface_config.width as f32, gpu.surface_config.height as f32),
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
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        render_pass.set_pipeline(&self.pipeline.inner);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..self.cell_count * 6, 0..1);
    }
}
