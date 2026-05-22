use crate::gpu::{Canvas, GpuContext};
use crate::stdlib::{State, StateRequest, InputManager};

pub struct TrigWaveDemoState {
    canvas: Canvas,
    time: f32,
}

impl TrigWaveDemoState {
    pub fn new(gpu: &GpuContext) -> Self {
        Self {
            canvas: Canvas::new(gpu),
            time: 0.0,
        }
    }
}

impl State for TrigWaveDemoState {
    fn update(&mut self, delta: f32, _input: &InputManager) -> Option<StateRequest> {
        self.time += delta;
        None
    }

    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let width = gpu.surface_config.width as f32;
        let height = gpu.surface_config.height as f32;
        let clear_color = [0.05, 0.05, 0.1, 1.0];

        self.canvas.draw_rectangle(0.0, 0.0, width, height, clear_color);

        self.canvas.draw_text(gpu, "Trigonometric Sine Wave", 20.0, 40.0, 30.0, [1.0, 1.0, 1.0, 1.0]);
        self.canvas.draw_text(gpu, "Formula: y = amplitude * sin(frequency * x + time)", 20.0, 70.0, 18.0, [0.7, 0.7, 0.7, 1.0]);

        let amplitude = 100.0;
        let frequency = 0.05;
        let center_y = height / 2.0;
        let step = 5.0;

        // Draw the wave using many small vertical lines (to simulate a continuous line)
        for x in (0..(width as i32)).step_by(step as usize) {
            let x_f = x as f32;
            let y = center_y + amplitude * (frequency * x_f + self.time).sin();
            
            // Draw a small vertical line for each point
            self.canvas.draw_rectangle(
                x_f, 
                y - 10.0, 
                2.0, 
                20.0, 
                [0.0, 1.0, 1.0, 1.0] // Cyan color
            );
        }

        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }

    fn handle_input(&mut self, key: winit::keyboard::KeyCode, state: winit::event::ElementState) -> Option<StateRequest> {
        if state == winit::event::ElementState::Pressed && key == winit::keyboard::KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }
}
