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
    fn update(&mut self, _delta: f32, _input: &InputManager) -> Option<StateRequest> {
        None
    }
    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
    fn handle_input(&mut self, _key: KeyCode, _state: ElementState) -> Option<StateRequest> {
        None
    }
    fn handle_char(&mut self, _c: char) -> Option<StateRequest> {
        None
    }
    fn handle_mouse_click(&mut self, _pos: [f32; 2], _sw: f32, _sh: f32) -> Option<StateRequest> {
        None
    }
    fn handle_mouse_wheel(&mut self, _delta_x: f32, _delta_y: f32) -> Option<StateRequest> {
        None
    }
    fn update_layout(&mut self, _sw: f32, _sh: f32) {}
}

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

    pub fn handle_mouse_click(&mut self, pos: [f32; 2], sw: f32, sh: f32) -> Option<StateRequest> {
        let request = self.current().handle_mouse_click(pos, sw, sh);
        request
    }

    pub fn handle_mouse_wheel(&mut self, delta_x: f32, delta_y: f32) -> Option<StateRequest> {
        let request = self.current().handle_mouse_wheel(delta_x, delta_y);
        request
    }

    pub fn update_layout(&mut self, sw: f32, sh: f32) {
        self.current().update_layout(sw, sh);
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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockState {
        id: i32,
    }

    impl State for MockState {
        fn render(&mut self, _gpu: &GpuContext, _encoder: &mut wgpu::CommandEncoder, _view: &wgpu::TextureView) {}
    }

    #[test]
    fn test_state_manager_stack() {
        let s1 = Box::new(MockState { id: 1 });
        let mut sm = StateManager::new(s1);
        
        assert!(!sm.is_empty());
        
        let s2 = Box::new(MockState { id: 2 });
        sm.handle_request(Some(StateRequest::Push(s2)));
        
        // Pop s2
        sm.handle_request(Some(StateRequest::Pop));
        
        // Pop s1
        sm.handle_request(Some(StateRequest::Pop));
        
        assert!(sm.is_empty());
    }

    #[test]
    fn test_state_manager_replace() {
        let s1 = Box::new(MockState { id: 1 });
        let mut sm = StateManager::new(s1);
        
        let s2 = Box::new(MockState { id: 2 });
        sm.handle_request(Some(StateRequest::Replace(s2)));
        
        // Replace should keep stack size at 1
        sm.handle_request(Some(StateRequest::Pop));
        assert!(sm.is_empty());
    }
}
