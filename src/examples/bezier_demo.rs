use crate::gpu::{Canvas, GpuContext};
use crate::stdlib::{State, StateRequest, InputManager};
use crate::stdlib::linear_algebra::vector::Vec2;
use winit::keyboard::KeyCode;
use winit::event::ElementState;

pub struct BezierDemoState {
    canvas: Canvas,
    points: Vec<Vec2>,
    selected_point: Option<usize>,
}

impl BezierDemoState {
    pub fn new(gpu: &GpuContext) -> Self {
        Self {
            canvas: Canvas::new(gpu),
            points: vec![
                Vec2::new(100.0, 500.0),
                Vec2::new(300.0, 100.0),
                Vec2::new(600.0, 100.0),
                Vec2::new(800.0, 500.0),
            ],
            selected_point: None,
        }
    }

    fn cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
        // De Casteljau's algorithm using lerp
        let q0 = p0.lerp(p1, t);
        let q1 = p1.lerp(p2, t);
        let q2 = p2.lerp(p3, t);
        
        let r0 = q0.lerp(q1, t);
        let r1 = q1.lerp(q2, t);
        
        r0.lerp(r1, t)
    }
}

impl State for BezierDemoState {
    fn update(&mut self, _delta: f32, input: &InputManager) -> Option<StateRequest> {
        if let Some(idx) = self.selected_point {
            // Assume we have access to mouse position. 
            // Since we don't have it in update, we'll handle dragging in handle_mouse_click 
            // or we'd need to add mouse_move to the State trait.
            // For now, let's just let the user "click" to move points.
        }
        None
    }

    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let sw = gpu.surface_config.width as f32;
        let sh = gpu.surface_config.height as f32;
        let clear_color = [0.05, 0.05, 0.1, 1.0];
        
        self.canvas.draw_rectangle(0.0, 0.0, sw, sh, clear_color);

        // 1. Draw the control polygon
        for i in 0..self.points.len() - 1 {
            let p0 = self.points[i];
            let p1 = self.points[i+1];
            // We don't have a draw_line in Canvas, but we can simulate it 
            // by drawing many small rectangles or using a different method.
            // Since Canvas is simple, let's just draw the points for now 
            // and focus on the curve.
        }

        // 2. Draw the curve
        let p0 = self.points[0];
        let p1 = self.points[1];
        let p2 = self.points[2];
        let p3 = self.points[3];
        
        let segments = 100;
        for i in 0..segments {
            let t0 = i as f32 / segments as f32;
            let t1 = (i + 1) as f32 / segments as f32;
            
            let pos0 = Self::cubic_bezier(p0, p1, p2, p3, t0);
            let pos1 = Self::cubic_bezier(p0, p1, p2, p3, t1);
            
            // Draw a small rectangle to simulate a line segment
            let mid = pos0.lerp(pos1, 0.5);
            self.canvas.draw_rectangle(mid.x - 1.0, mid.y - 1.0, 2.0, 2.0, [1.0, 1.0, 1.0, 1.0]);
        }

        // 3. Draw control points
        for (i, p) in self.points.iter().enumerate() {
            let color = if Some(i) == self.selected_point { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 1.0, 1.0, 1.0] };
            self.canvas.draw_rectangle(p.x - 5.0, p.y - 5.0, 10.0, 10.0, color);
            self.canvas.draw_text(gpu, &format!("P{}", i), p.x + 10.0, p.y - 10.0, 14.0, [0.8, 0.8, 0.8, 1.0]);
        }

        self.canvas.draw_text(gpu, "Bézier Curve Editor", 20.0, 40.0, 24.0, [1.0, 1.0, 1.0, 1.0]);
        self.canvas.draw_text(gpu, "Click a point to select | ESC to return", 20.0, 70.0, 16.0, [0.6, 0.6, 0.6, 1.0]);

        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        if state == ElementState::Pressed && key == KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }

    fn handle_mouse_click(&mut self, pos: [f32; 2], _sw: f32, _sh: f32) -> Option<StateRequest> {
        let click_pos = Vec2::new(pos[0], pos[1]);
        for (i, p) in self.points.iter().enumerate() {
            if (p.x - click_pos.x).abs() < 10.0 && (p.y - click_pos.y).abs() < 10.0 {
                self.selected_point = Some(i);
                // Move the point to the click position
                self.points[i] = click_pos;
                return None;
            }
        }
        self.selected_point = None;
        None
    }
}
