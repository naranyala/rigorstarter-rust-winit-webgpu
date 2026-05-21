use winit::keyboard::KeyCode;
use crate::gpu::{Canvas, GpuContext};

pub struct KeyIndicator {
    pub last_key: Option<KeyCode>,
}

impl KeyIndicator {
    pub fn new() -> Self {
        Self { last_key: None }
    }

    pub fn update(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            self.last_key = Some(key);
        } else {
            self.last_key = None;
        }
    }
}

pub struct KeyIndicatorRenderer {
    canvas: Canvas,
}

impl KeyIndicatorRenderer {
    pub fn new(gpu: &GpuContext) -> Self {
        Self {
            canvas: Canvas::new(gpu),
        }
    }

    pub fn render(
        &mut self,
        gpu: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        game: &KeyIndicator,
        width: u32,
        height: u32,
    ) {
        let clear_color = [0.05, 0.05, 0.1, 1.0];
        
        if let Some(key) = game.last_key {
            let key_str = format!("{:?}", key);
            self.canvas.draw_rectangle(0.0, 0.0, width as f32, height as f32, clear_color);
            self.canvas.draw_text(gpu, &key_str, width as f32 / 2.0 - 50.0, height as f32 / 2.0, 40.0, [1.0, 1.0, 1.0, 1.0]);
        } else {
            self.canvas.draw_rectangle(0.0, 0.0, width as f32, height as f32, clear_color);
            self.canvas.draw_text(gpu, "Press any key", width as f32 / 2.0 - 50.0, height as f32 / 2.0, 40.0, [1.0, 1.0, 1.0, 1.0]);
        }

        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }
}
