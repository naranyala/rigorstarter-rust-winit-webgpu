use crate::gpu::{Canvas, GpuContext};
use winit::keyboard::KeyCode;
use winit::event::ElementState;
use crate::stdlib::{State, StateRequest, InputManager};

#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    pub radius: f32,
}

pub struct Brick {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub active: bool,
}

pub struct BreakoutsGame {
    pub ball: Ball,
    pub paddle: [f32; 2], // x, width
    pub bricks: Vec<Brick>,
    pub score: u32,
    pub game_over: bool,
    pub won: bool,
}

impl BreakoutsGame {
    pub fn new() -> Self {
        let mut bricks = Vec::new();
        let rows = 5;
        let cols = 8;
        let brick_w = 100.0;
        let brick_h = 20.0;
        let gap = 5.0;
        let offset_x = (900.0 - (cols as f32 * (brick_w + gap))) / 2.0;
        let offset_y = 50.0;

        for r in 0..rows {
            for c in 0..cols {
                bricks.push(Brick {
                    pos: [
                        offset_x + c as f32 * (brick_w + gap),
                        offset_y + r as f32 * (brick_h + gap),
                    ],
                    size: [brick_w, brick_h],
                    active: true,
                });
            }
        }

        Self {
            ball: Ball {
                pos: [450.0, 500.0],
                vel: [200.0, -200.0],
                radius: 6.0,
            },
            paddle: [400.0, 100.0],
            bricks,
            score: 0,
            game_over: false,
            won: false,
        }
    }

    pub fn update(&mut self, delta: f32, input: &InputManager) {
        if self.game_over { return; }

        // Paddle movement
        let mut move_dir = 0.0;
        if input.is_down(KeyCode::ArrowLeft) || input.is_down(KeyCode::KeyA) {
            move_dir -= 1.0;
        }
        if input.is_down(KeyCode::ArrowRight) || input.is_down(KeyCode::KeyD) {
            move_dir += 1.0;
        }
        self.paddle[0] = (self.paddle[0] + move_dir * 500.0 * delta).clamp(0.0, 900.0 - self.paddle[1]);

        // Ball movement
        self.ball.pos[0] += self.ball.vel[0] * delta;
        self.ball.pos[1] += self.ball.vel[1] * delta;

        if self.ball.pos[0] - self.ball.radius < 0.0 || self.ball.pos[0] + self.ball.radius > 900.0 {
            self.ball.vel[0] *= -1.0;
        }
        if self.ball.pos[1] - self.ball.radius < 0.0 {
            self.ball.vel[1] *= -1.0;
        }
        if self.ball.pos[1] + self.ball.radius > 600.0 {
            self.game_over = true;
        }

        // Paddle Collision
        let paddle_rect = crate::stdlib::Rect {
            x: self.paddle[0],
            y: 550.0,
            w: self.paddle[1],
            h: 10.0,
        };
        let ball_circle = crate::stdlib::Circle {
            x: self.ball.pos[0],
            y: self.ball.pos[1],
            radius: self.ball.radius,
        };

        if crate::stdlib::aabb_vs_circle(&paddle_rect, &ball_circle) {
            self.ball.vel[1] *= -1.0;
            let hit_pos = (self.ball.pos[0] - (self.paddle[0] + self.paddle[1] / 2.0)) / (self.paddle[1] / 2.0);
            self.ball.vel[0] += hit_pos * 100.0;
        }

        // Brick Collision
        for brick in self.bricks.iter_mut().filter(|b| b.active) {
            let brick_rect = crate::stdlib::Rect {
                x: brick.pos[0],
                y: brick.pos[1],
                w: brick.size[0],
                h: brick.size[1],
            };

            if crate::stdlib::aabb_vs_circle(&brick_rect, &ball_circle) {
                brick.active = false;
                self.ball.vel[1] *= -1.0;
                self.score += 10;
                break; 
            }
        }

        if self.bricks.iter().all(|b| !b.active) {
            self.won = true;
            self.game_over = true;
        }
    }

    pub fn reset(&mut self) {
        self.ball.pos = [450.0, 500.0];
        self.ball.vel = [200.0, -200.0];
        for b in &mut self.bricks { b.active = true; }
        self.game_over = false;
        self.won = false;
    }
}

pub struct BreakoutsRenderer {
    canvas: Canvas,
}

impl BreakoutsRenderer {
    pub fn new(gpu: &GpuContext) -> Self {
        Self {
            canvas: Canvas::new(gpu),
        }
    }

    pub fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, game: &BreakoutsGame) {
        let clear_color = [0.05, 0.05, 0.1, 1.0];
        
        // Score
        let score_text = format!("Score: {}", game.score);
        self.canvas.draw_text(gpu, &score_text, 20.0, 20.0, 20.0, [0.8, 0.8, 0.8, 1.0]);

        // Paddle
        self.canvas.draw_rectangle(game.paddle[0], 550.0, game.paddle[1], 10.0, [0.2, 1.0, 0.3, 1.0]);
        
        // Ball
        self.canvas.draw_circle(game.ball.pos[0], game.ball.pos[1], game.ball.radius, [1.0, 1.0, 1.0, 1.0]);
        
        // Bricks
        for brick in &game.bricks {
            if brick.active {
                self.canvas.draw_rectangle(brick.pos[0], brick.pos[1], brick.size[0], brick.size[1], [0.8, 0.2, 0.2, 1.0]);
            }
        }
        if game.game_over {
            let text = if game.won { "YOU WIN!" } else { "GAME OVER" };
            self.canvas.draw_text(gpu, text, 400.0, 300.0, 40.0, [1.0, 1.0, 1.0, 1.0]);
            self.canvas.draw_text(gpu, "Press ESC to return", 360.0, 340.0, 20.0, [0.7, 0.7, 0.7, 1.0]);
        }
        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }
}

pub struct BreakoutsState {
    game: BreakoutsGame,
    renderer: BreakoutsRenderer,
}

impl BreakoutsState {
    pub fn new(gpu: &GpuContext) -> Self {
        let game = BreakoutsGame::new();
        let renderer = BreakoutsRenderer::new(gpu);
        Self { game, renderer }
    }
}

impl State for BreakoutsState {
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
