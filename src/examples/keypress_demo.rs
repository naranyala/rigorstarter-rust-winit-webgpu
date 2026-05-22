use winit::event::ElementState;
use winit::keyboard::KeyCode;
use crate::gpu::{Canvas, GpuContext};
use crate::stdlib::{State, StateRequest, InputManager};

struct KeyPressDemo {
    history: Vec<KeyCode>,
}

impl KeyPressDemo {
    fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    fn handle_key(&mut self, key: KeyCode, state: ElementState) {
        if state == ElementState::Pressed {
            self.history.push(key);
            if self.history.len() > 10 {
                self.history.remove(0);
            }
        }
    }
}

struct KeyPressDemoRenderer {
    canvas: Canvas,
}

impl KeyPressDemoRenderer {
    fn new(gpu: &GpuContext) -> Self {
        Self {
            canvas: Canvas::new(gpu),
        }
    }

    fn render(
        &mut self,
        gpu: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        demo: &KeyPressDemo,
        width: u32,
        height: u32,
    ) {
        let clear_color = [0.05, 0.05, 0.1, 1.0];
        self.canvas.draw_rectangle(0.0, 0.0, width as f32, height as f32, clear_color);

        self.canvas.draw_text(gpu, "Key Press History:", 20.0, 40.0, 30.0, [1.0, 1.0, 1.0, 1.0]);

        for (i, key) in demo.history.iter().enumerate() {
            let key_str = format!("{:?}", key);
            self.canvas.draw_text(gpu, &key_str, 20.0, 80.0 + (i as f32 * 40.0), 25.0, [0.8, 0.8, 0.8, 1.0]);
        }

        if demo.history.is_empty() {
            self.canvas.draw_text(gpu, "Press any key...", 20.0, 80.0, 25.0, [0.5, 0.5, 0.5, 1.0]);
        }

        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }
}

pub struct KeyPressDemoState {
    demo: KeyPressDemo,
    renderer: KeyPressDemoRenderer,
}

impl KeyPressDemoState {
    pub fn new(gpu: &GpuContext) -> Self {
        Self {
            demo: KeyPressDemo::new(),
            renderer: KeyPressDemoRenderer::new(gpu),
        }
    }
}

impl State for KeyPressDemoState {
    fn update(&mut self, _delta: f32, _input: &InputManager) -> Option<StateRequest> {
        None
    }

    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let width = gpu.surface_config.width;
        let height = gpu.surface_config.height;
        self.renderer.render(gpu, encoder, view, &self.demo, width, height);
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        self.demo.handle_key(key, state);
        if state == ElementState::Pressed && key == KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }
}
