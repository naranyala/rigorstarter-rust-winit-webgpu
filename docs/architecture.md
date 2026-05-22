# Architecture

The project follows a state-driven architecture designed for interactive applications and games.

## State Pattern

At the core of the application is the `State` trait. Every screen or game mode is implemented as a `State`.

```rust
pub trait State {
    fn update(&mut self, delta: f32, input: &InputManager) -> Option<StateRequest>;
    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest>;
    fn handle_char(&mut self, c: char) -> Option<StateRequest>;
    fn handle_mouse_click(&mut self, pos: [f32; 2], sw: f32, sh: f32) -> Option<StateRequest>;
    fn handle_mouse_wheel(&mut self, delta_x: f32, delta_y: f32) -> Option<StateRequest>;
}
```

## State Manager

The `StateManager` manages a stack of `State` objects. This allows for easy navigation, such as pushing a game state on top of a menu state, and popping it to return to the menu.

- `Push`: Adds a new state to the top of the stack.
- `Pop`: Removes the current state and returns to the previous one.
- `Replace`: Replaces the current state with a new one.

## GpuContext

The `GpuContext` encapsulates the WebGPU device, queue, and surface configuration. It provides a unified way to access GPU resources across different states.
