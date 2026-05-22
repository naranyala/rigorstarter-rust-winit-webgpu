pub mod snake;
pub mod breakouts;
pub mod pingpong;

use crate::gpu::GpuContext;
use crate::stdlib::{State, StateRequest, InputManager};
use winit::keyboard::KeyCode;
use winit::event::ElementState;

pub struct SnakeState {
    game: snake::SnakeGame,
    renderer: snake::SnakeRenderer,
}
impl SnakeState {
    pub fn new(gpu: &GpuContext) -> Self {
        let game = snake::SnakeGame::new(30, 20);
        let renderer = snake::SnakeRenderer::new(gpu, &game);
        Self { game, renderer }
    }
}
impl State for SnakeState {
    fn update(&mut self, delta: f32, _input: &InputManager) -> Option<StateRequest> {
        self.game.update(delta as f64);
        None
    }
    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        self.renderer.update(gpu, &self.game);
        self.renderer.render(encoder, view);
    }
    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        if state == ElementState::Pressed {
            match key {
                KeyCode::ArrowUp | KeyCode::KeyW => self.game.set_direction(snake::Direction::Up),
                KeyCode::ArrowDown | KeyCode::KeyS => self.game.set_direction(snake::Direction::Down),
                KeyCode::ArrowLeft | KeyCode::KeyA => self.game.set_direction(snake::Direction::Left),
                KeyCode::ArrowRight | KeyCode::KeyD => self.game.set_direction(snake::Direction::Right),
                KeyCode::Space if self.game.game_over => self.game.reset(),
                KeyCode::Escape => return Some(StateRequest::Pop),
                _ => {}
            }
        }
        None
    }
}

pub struct LinearAlgebraState {
    renderer: crate::gpu::Canvas,
}
impl LinearAlgebraState {
    pub fn new(gpu: &GpuContext) -> Self {
        println!("Launching Linear Algebra Demo...");
        Self {
            renderer: crate::gpu::Canvas::new(gpu),
        }
    }
}
impl State for LinearAlgebraState {
    fn update(&mut self, _delta: f32, _input: &InputManager) -> Option<StateRequest> {
        None
    }
    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let sw = gpu.surface_config.width as f32;
        let sh = gpu.surface_config.height as f32;
        self.renderer.draw_text(gpu, "Linear Algebra Demo", sw / 2.0 - 150.0, sh / 2.0, 30.0, [1.0, 1.0, 1.0, 1.0]);
        self.renderer.draw_text(gpu, "Check console for output!", sw / 2.0 - 120.0, sh / 2.0 + 40.0, 20.0, [0.7, 0.7, 0.7, 1.0]);
        self.renderer.end_drawing(gpu, encoder, view, [0.1, 0.1, 0.2, 1.0]);
    }
    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        if state == ElementState::Pressed && key == KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }
}
