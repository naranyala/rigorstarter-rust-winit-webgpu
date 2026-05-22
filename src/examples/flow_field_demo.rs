use crate::gpu::{Canvas, GpuContext};
use crate::stdlib::{State, StateRequest, InputManager};
use crate::stdlib::linear_algebra::vector::Vec2;
use winit::keyboard::KeyCode;
use winit::event::ElementState;
use rand::Rng;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max_life: f32,
}

impl Particle {
    fn new(sw: f32, sh: f32) -> Self {
        let mut rng = rand::thread_rng();
        let life = rng.gen_range(1.0..5.0);
        Self {
            pos: Vec2::new(rng.gen_range(0.0..sw), rng.gen_range(0.0..sh)),
            vel: Vec2::ZERO,
            life,
            max_life: life,
        }
    }

    fn reset(&mut self, sw: f32, sh: f32) {
        let mut rng = rand::thread_rng();
        self.life = rng.gen_range(1.0..5.0);
        self.max_life = self.life;
        self.pos = Vec2::new(rng.gen_range(0.0..sw), rng.gen_range(0.0..sh));
        self.vel = Vec2::ZERO;
    }
}

pub struct FlowFieldDemoState {
    canvas: Canvas,
    particles: Vec<Particle>,
    time: f32,
    attractor: Vec2,
    attractor_active: bool,
}

impl FlowFieldDemoState {
    pub fn new(gpu: &GpuContext) -> Self {
        let sw = gpu.surface_config.width as f32;
        let sh = gpu.surface_config.height as f32;
        
        let particles = (0..1000)
            .map(|_| Particle::new(sw, sh))
            .collect();

        Self {
            canvas: Canvas::new(gpu),
            particles,
            time: 0.0,
            attractor: Vec2::ZERO,
            attractor_active: false,
        }
    }

    fn sample_field(pos: Vec2, time: f32) -> Vec2 {
        let scale = 0.005;
        // Create a swirling flow field using sine/cosine
        let angle = (pos.x * scale).sin() * 2.0 * std::f32::consts::PI 
                  + (pos.y * scale).cos() * 2.0 * std::f32::consts::PI 
                  + time;
        
        Vec2::new(angle.cos(), angle.sin())
    }
}

impl State for FlowFieldDemoState {
    fn update(&mut self, delta: f32, _input: &InputManager) -> Option<StateRequest> {
        self.time += delta * 0.2;
        
        let sw = 900.0; // Simplified, should probably be passed in or stored
        let sh = 600.0;

        for p in &mut self.particles {
            // 1. Sample the field
            let force = Self::sample_field(p.pos, self.time);
            
            // 2. Handle attractor
            if self.attractor_active {
                let to_attractor = (self.attractor - p.pos).normalize();
                let dist = (self.attractor - p.pos).length();
                let attract_strength = (1.0 / (dist * 0.01 + 1.0)).min(2.0);
                
                // Blend field flow with attractor pull
                let combined_force = (force * 0.7 + to_attractor * attract_strength * 0.3).normalize();
                p.vel = p.vel * 0.95 + combined_force * delta * 50.0;
            } else {
                p.vel = p.vel * 0.95 + force * delta * 50.0;
            }

            // 3. Update position
            p.pos = p.pos + p.vel * delta;
            p.life -= delta;

            // 4. Bounds and life check
            if p.life <= 0.0 || p.pos.x < 0.0 || p.pos.x > sw || p.pos.y < 0.0 || p.pos.y > sh {
                p.reset(sw, sh);
            }
        }
        None
    }

    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let sw = gpu.surface_config.width as f32;
        let sh = gpu.surface_config.height as f32;
        let clear_color = [0.02, 0.02, 0.05, 1.0];
        
        self.canvas.draw_rectangle(0.0, 0.0, sw, sh, clear_color);

        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let color = [0.4, 0.7, 1.0, alpha];
            
            // Draw particle as a small square
            self.canvas.draw_rectangle(p.pos.x, p.pos.y, 2.0, 2.0, color);
        }

        if self.attractor_active {
            self.canvas.draw_rectangle(self.attractor.x - 5.0, self.attractor.y - 5.0, 10.0, 10.0, [1.0, 0.3, 0.3, 1.0]);
        }

        self.canvas.draw_text(gpu, "Particle Flow Field", 20.0, 40.0, 24.0, [1.0, 1.0, 1.0, 1.0]);
        self.canvas.draw_text(gpu, "Click and hold to attract particles | ESC to return", 20.0, 70.0, 16.0, [0.6, 0.6, 0.6, 1.0]);

        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        if state == ElementState::Pressed && key == KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }

    fn handle_mouse_click(&mut self, pos: [f32; 2], _sw: f32, _sh: f32) -> Option<StateRequest> {
        // Since we can't easily detect "mouse release" in the current State trait 
        // without adding a method, we'll just toggle it or handle it in a different way.
        // For now, we'll just set the attractor position.
        self.attractor_active = true;
        self.attractor = Vec2::new(pos[0], pos[1]);
        None
    }
}
