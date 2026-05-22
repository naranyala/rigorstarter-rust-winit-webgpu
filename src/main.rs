use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent, MouseScrollDelta};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowAttributes};

use rigorstarter_rust_tauri_webgpu::gpu::GpuContext;
use rigorstarter_rust_tauri_webgpu::ui::launcher::{LauncherRenderer, LauncherState};
use rigorstarter_rust_tauri_webgpu::stdlib::{Clock, InputManager, State, StateManager, StateRequest};


const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 600;

/// Wrapper to make the Launcher a State
struct LauncherStateWrapper {
    state: LauncherState,
    renderer: LauncherRenderer,
    gpu: Arc<GpuContext>,
}

impl State for LauncherStateWrapper {
    fn update(&mut self, delta: f32, _input: &InputManager) -> Option<StateRequest> {
        self.state.update(delta as f64);
        None
    }

    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        self.renderer.render(gpu, encoder, view, &self.state);
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        self.state.handle_input(key, state, &self.gpu)
    }

    fn handle_char(&mut self, c: char) -> Option<StateRequest> {
        self.state.input_char(c);
        None
    }

    fn handle_mouse_click(&mut self, pos: [f32; 2], sw: f32, sh: f32) -> Option<StateRequest> {
        self.state.handle_mouse_click(pos, sw, sh, &self.gpu)
    }

    fn update_layout(&mut self, sw: f32, sh: f32) {
        self.state.update_layout(sw, sh);
    }
}

struct App {
    window: Option<Arc<Window>>,
    gpu: Option<Arc<GpuContext>>,
    state_manager: Option<StateManager>,
    clock: Clock,
    input: InputManager,
    cursor_pos: [f32; 2],
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            gpu: None,
            state_manager: None,
            clock: Clock::new(),
            input: InputManager::new(),
            cursor_pos: [0.0, 0.0],
        }
    }

    async fn init(&mut self, window: Arc<Window>) {
        let gpu = Arc::new(GpuContext::new(window.clone(), WINDOW_WIDTH, WINDOW_HEIGHT).await);
        
        let mut launcher = LauncherState::new();
        launcher.update_layout(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
        let launcher_renderer = LauncherRenderer::new(&gpu);
        let initial_state = Box::new(LauncherStateWrapper {
            state: launcher,
            renderer: launcher_renderer,
            gpu: gpu.clone(),
        });

        self.window = Some(window);
        self.gpu = Some(gpu);
        self.state_manager = Some(StateManager::new(initial_state));
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) {
        self.input.update_key(key, state);

        if let Some(sm) = &mut self.state_manager {
            let request = sm.handle_input(key, state);
            sm.handle_request(request);
        }
    }

    fn render(&mut self) {
        self.clock.tick();
        self.input.tick();
        let delta = self.clock.delta_time();

        if let (Some(gpu), Some(sm), Some(window)) =
            (&self.gpu, &mut self.state_manager, &self.window)
        {
            if let Some(frame) = gpu.get_frame() {
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                
                let request = sm.update(delta, &self.input);
                sm.handle_request(request);
                sm.render(gpu, &mut encoder, &view);
                
                gpu.queue.submit(std::iter::once(encoder.finish()));
                frame.present();
            }
            window.request_redraw();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Rigorstarter - Winit + WebGPU")
                        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                )
                .unwrap();
            let window = Arc::new(window);
            pollster::block_on(self.init(window));
            self.window.as_ref().unwrap().request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    if let Some(gpu_mut) = Arc::get_mut(gpu) {
                        gpu_mut.resize(size.width, size.height);
                    }
                }
                if let Some(sm) = &mut self.state_manager {
                    sm.update_layout(size.width as f32, size.height as f32);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = [position.x as f32, position.y as f32];
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if let (Some(gpu), Some(sm)) = (&self.gpu, &mut self.state_manager) {
                    let sw = gpu.surface_config.width as f32;
                    let sh = gpu.surface_config.height as f32;
                    let request = sm.current().handle_mouse_click(self.cursor_pos, sw, sh);
                    sm.handle_request(request);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(sm) = &mut self.state_manager {
                    let request = match delta {
                        MouseScrollDelta::LineDelta(x, y) => sm.handle_mouse_wheel(x, y),
                        MouseScrollDelta::PixelDelta(pos) => sm.handle_mouse_wheel(pos.x as f32, pos.y as f32),
                    };
                    sm.handle_request(request);
                }
            }
            WindowEvent::KeyboardInput {
                event,
                ..
            } => {
                let KeyEvent {
                    state,
                    physical_key,
                    text,
                    ..
                } = event;
                if let winit::keyboard::PhysicalKey::Code(code) = physical_key {
                    self.handle_input(code, state);
                }
                if let Some(text) = text {
                    for c in text.chars() {
                        if let Some(sm) = &mut self.state_manager {
                            let request = sm.handle_char(c);
                            sm.handle_request(request);
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
