use crate::gpu::{Canvas, GpuContext};
use winit::keyboard::KeyCode;
use winit::event::ElementState;
use crate::stdlib::{State, StateRequest, InputManager};

pub struct PingPongGame {
    pub ball_pos: [f32; 2],
    pub ball_vel: [f32; 2],
    pub p1_y: f32,
    pub p2_y: f32,
    pub p1_score: u32,
    pub p2_score: u32,
}

impl PingPongGame {
    pub fn new() -> Self {
        Self {
            ball_pos: [450.0, 300.0],
            ball_vel: [300.0, 200.0],
            p1_y: 250.0,
            p2_y: 250.0,
            p1_score: 0,
            p2_score: 0,
        }
    }

    pub fn update(&mut self, delta: f32, input: &InputManager) {
        let mut p1_move = 0.0;
        if input.is_down(KeyCode::ArrowUp) || input.is_down(KeyCode::KeyW) {
            p1_move -= 1.0;
        }
        if input.is_down(KeyCode::ArrowDown) || input.is_down(KeyCode::KeyS) {
            p1_move += 1.0;
        }
        self.p1_y += p1_move * 400.0 * delta;
        self.p1_y = self.p1_y.clamp(0.0, 600.0 - 100.0);

        let p2_center = self.p2_y + 50.0;
        if self.ball_pos[1] > p2_center {
            self.p2_y += 200.0 * delta;
        } else {
            self.p2_y -= 200.0 * delta;
        }
        self.p2_y = self.p2_y.clamp(0.0, 600.0 - 100.0);

        self.ball_pos[0] += self.ball_vel[0] * delta;
        self.ball_pos[1] += self.ball_vel[1] * delta;

        if self.ball_pos[1] < 0.0 || self.ball_pos[1] > 600.0 {
            self.ball_vel[1] *= -1.0;
        }

        let p1_rect = crate::stdlib::Rect {
            x: 10.0,
            y: self.p1_y,
            w: 10.0,
            h: 100.0,
        };
        let ball_circle = crate::stdlib::Circle {
            x: self.ball_pos[0],
            y: self.ball_pos[1],
            radius: 6.0,
        };

        if crate::stdlib::aabb_vs_circle(&p1_rect, &ball_circle) {
            self.ball_vel[0] *= -1.0;
        }

        let p2_rect = crate::stdlib::Rect {
            x: 890.0 - 10.0,
            y: self.p2_y,
            w: 10.0,
            h: 100.0,
        };

        if crate::stdlib::aabb_vs_circle(&p2_rect, &ball_circle) {
            self.ball_vel[0] *= -1.0;
        }

        if self.ball_pos[0] < 0.0 {
            self.p2_score += 1;
            self.reset_ball();
        } else if self.ball_pos[0] > 900.0 {
            self.p1_score += 1;
            self.reset_ball();
        }
    }

    pub fn move_p1(&mut self, delta: f32) {
        self.p1_y += delta * 400.0 * 0.016;
        self.p1_y = self.p1_y.clamp(0.0, 600.0 - 100.0);
    }

    fn reset_ball(&mut self) {
        self.ball_pos = [450.0, 300.0];
        self.ball_vel[0] *= -1.0;
    }
}

pub struct PingPongState {
    game: PingPongGame,
    renderer: PingPongRenderer,
}

impl PingPongState {
    pub fn new(gpu: &GpuContext) -> Self {
        let game = PingPongGame::new();
        let renderer = PingPongRenderer::new(gpu);
        Self { game, renderer }
    }
}

impl State for PingPongState {
    fn update(&mut self, delta: f32, input: &InputManager) -> Option<StateRequest> {
        self.game.update(delta, input);
        None
    }

    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        self.renderer.render(gpu, encoder, view, &self.game);
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        if state == ElementState::Pressed && key == KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }
}

pub struct PingPongRenderer {
    canvas: Canvas,
}

impl PingPongRenderer {
    pub fn new(gpu: &GpuContext) -> Self {
        Self {
            canvas: Canvas::new(gpu),
        }
    }

    pub fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, game: &PingPongGame) {
        let clear_color = [0.05, 0.05, 0.1, 1.0];
        
        // Score
        let s1 = format!("Player 1: {}", game.p1_score);
        let s2 = format!("Player 2: {}", game.p2_score);
        self.canvas.draw_text(gpu, &s1, 300.0, 50.0, 30.0, [1.0, 1.0, 1.0, 1.0]);
        self.canvas.draw_text(gpu, &s2, 550.0, 50.0, 30.0, [1.0, 1.0, 1.0, 1.0]);

        // Paddles
        self.canvas.draw_rectangle(10.0, game.p1_y, 10.0, 100.0, [1.0, 1.0, 1.0, 1.0]);
        self.canvas.draw_rectangle(890.0 - 10.0, game.p2_y, 10.0, 100.0, [1.0, 1.0, 1.0, 1.0]);
        
        // Center line
        self.canvas.draw_rectangle(449.0, 0.0, 2.0, 600.0, [0.3, 0.3, 0.4, 1.0]);
        
        // Ball
        self.canvas.draw_circle(game.ball_pos[0], game.ball_pos[1], 6.0, [1.0, 1.0, 1.0, 1.0]);
        
        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }
}
