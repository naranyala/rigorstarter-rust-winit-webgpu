use crate::gpu::GpuContext;
use crate::stdlib::input::InputManager;
use winit::keyboard::KeyCode;
use winit::event::ElementState;

/// Requests a state transition
pub enum StateRequest {
    Push(Box<dyn State>),
    Pop,
    Replace(Box<dyn State>),
    Quit,
}

/// The interface every game/screen must implement
pub trait State {
    fn update(&mut self, delta: f32, input: &InputManager) -> Option<StateRequest> {
        None
    }
    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        None
    }
    fn handle_char(&mut self, c: char) -> Option<StateRequest> {
        None
    }
    fn handle_mouse_click(&mut self, pos: [f32; 2], sw: f32, sh: f32) -> Option<StateRequest> {
        None
    }}

pub struct StateManager {
    stack: Vec<Box<dyn State>>,
}

impl StateManager {
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            stack: vec![initial_state],
        }
    }

    pub fn current(&mut self) -> &mut Box<dyn State> {
        self.stack.last_mut().unwrap()
    }

    pub fn update(&mut self, delta: f32, input: &InputManager) -> Option<StateRequest> {
        let request = self.current().update(delta, input);
        if let Some(ref req) = request {
            // We can't clone StateRequest, so we handle it here or return it
            // But wait, we need to apply the request.
        }
        // Since we can't clone, we'll just return it and let the caller handle it,
        // or handle it inside update.
        request
    }

    pub fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        self.current().render(gpu, encoder, view);
    }

    pub fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        let request = self.current().handle_input(key, state);
        request
    }

    pub fn handle_char(&mut self, c: char) -> Option<StateRequest> {
        let request = self.current().handle_char(c);
        request
    }

    pub fn handle_request(&mut self, request: Option<StateRequest>) {
        if let Some(req) = request {
            match req {
                StateRequest::Push(state) => self.stack.push(state),
                StateRequest::Pop => { self.stack.pop(); },
                StateRequest::Replace(state) => {
                    self.stack.pop();
                    self.stack.push(state);
                },
                StateRequest::Quit => {
                    // Handled by the main loop
                },
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
