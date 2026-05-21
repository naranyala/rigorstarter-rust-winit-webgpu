mod game;
mod gpu;
mod ui;

use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowAttributes};

use game::renderer::SnakeRenderer;
use game::snake::{Direction, SnakeGame};
use gpu::GpuContext;
use ui::launcher::{LauncherRenderer, LauncherState};

const GRID_WIDTH: i32 = 30;
const GRID_HEIGHT: i32 = 20;
const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 600;

enum AppState {
    Launcher {
        launcher: LauncherState,
        launcher_renderer: LauncherRenderer,
    },
    Snake {
        game: SnakeGame,
        renderer: SnakeRenderer,
    },
}

struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    state: Option<AppState>,
    last_time: Option<Instant>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            gpu: None,
            state: None,
            last_time: None,
        }
    }

    async fn init(&mut self, window: Arc<Window>) {
        let gpu = GpuContext::new(window.clone(), WINDOW_WIDTH, WINDOW_HEIGHT).await;
        let launcher = LauncherState::new();
        let launcher_renderer = LauncherRenderer::new(&gpu);

        self.window = Some(window);
        self.gpu = Some(gpu);
        self.state = Some(AppState::Launcher {
            launcher,
            launcher_renderer,
        });
        self.last_time = Some(Instant::now());
    }

    fn handle_input(&mut self, key: KeyCode) {
        match &mut self.state {
            Some(AppState::Launcher { launcher, .. }) => match key {
                KeyCode::Escape => {
                    launcher.search_text.clear();
                    launcher.selected_index = 0;
                }
                KeyCode::Enter => {
                    let filtered = launcher.filtered_items();
                    if let Some((idx, _)) = filtered.get(launcher.selected_index) {
                        if *idx == 0 {
                            self.start_snake_game();
                        }
                    }
                }
                KeyCode::ArrowUp => launcher.move_selection(-1),
                KeyCode::ArrowDown => launcher.move_selection(1),
                KeyCode::Backspace => launcher.backspace(),
                KeyCode::KeyA => launcher.input_char('a'),
                KeyCode::KeyB => launcher.input_char('b'),
                KeyCode::KeyC => launcher.input_char('c'),
                KeyCode::KeyD => launcher.input_char('d'),
                KeyCode::KeyE => launcher.input_char('e'),
                KeyCode::KeyF => launcher.input_char('f'),
                KeyCode::KeyG => launcher.input_char('g'),
                KeyCode::KeyH => launcher.input_char('h'),
                KeyCode::KeyI => launcher.input_char('i'),
                KeyCode::KeyJ => launcher.input_char('j'),
                KeyCode::KeyK => launcher.input_char('k'),
                KeyCode::KeyL => launcher.input_char('l'),
                KeyCode::KeyM => launcher.input_char('m'),
                KeyCode::KeyN => launcher.input_char('n'),
                KeyCode::KeyO => launcher.input_char('o'),
                KeyCode::KeyP => launcher.input_char('p'),
                KeyCode::KeyQ => launcher.input_char('q'),
                KeyCode::KeyR => launcher.input_char('r'),
                KeyCode::KeyS => launcher.input_char('s'),
                KeyCode::KeyT => launcher.input_char('t'),
                KeyCode::KeyU => launcher.input_char('u'),
                KeyCode::KeyV => launcher.input_char('v'),
                KeyCode::KeyW => launcher.input_char('w'),
                KeyCode::KeyX => launcher.input_char('x'),
                KeyCode::KeyY => launcher.input_char('y'),
                KeyCode::KeyZ => launcher.input_char('z'),
                _ => {}
            },
            Some(AppState::Snake { game, .. }) => match key {
                KeyCode::ArrowUp | KeyCode::KeyW => game.set_direction(Direction::Up),
                KeyCode::ArrowDown | KeyCode::KeyS => game.set_direction(Direction::Down),
                KeyCode::ArrowLeft | KeyCode::KeyA => game.set_direction(Direction::Left),
                KeyCode::ArrowRight | KeyCode::KeyD => game.set_direction(Direction::Right),
                KeyCode::Space if game.game_over => game.reset(),
                KeyCode::Escape => {
                    self.state = Some(self.take_launcher_state());
                }
                _ => {}
            },
            None => {}
        }
    }

    fn start_snake_game(&mut self) {
        if let (Some(gpu), Some(_)) = (&self.gpu, &self.state) {
            let game = SnakeGame::new(GRID_WIDTH, GRID_HEIGHT);
            let renderer = SnakeRenderer::new(gpu, &game);
            self.state = Some(AppState::Snake { game, renderer });
        }
    }

    fn take_launcher_state(&mut self) -> AppState {
        if let Some(gpu) = &self.gpu {
            let launcher = LauncherState::new();
            let launcher_renderer = LauncherRenderer::new(gpu);
            AppState::Launcher {
                launcher,
                launcher_renderer,
            }
        } else {
            unreachable!()
        }
    }

    fn render(&mut self) {
        let now = Instant::now();
        let delta = now
            .duration_since(self.last_time.unwrap_or(now))
            .as_secs_f64();
        self.last_time = Some(now);

        if let (Some(gpu), Some(state), Some(window)) =
            (&mut self.gpu, &mut self.state, &self.window)
        {
            match state {
                AppState::Launcher {
                    launcher,
                    launcher_renderer,
                } => {
                    launcher.update(delta);
                    if let Some(frame) = gpu.get_frame() {
                        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        launcher_renderer.render(gpu, &view, launcher);
                        frame.present();
                    }
                }
                AppState::Snake { game, renderer } => {
                    game.update(delta);
                    renderer.update(gpu, game);

                    if let Some(frame) = gpu.get_frame() {
                        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        renderer.render(&mut encoder, &view);
                        gpu.queue.submit(std::iter::once(encoder.finish()));
                        frame.present();
                    }
                }
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
                    gpu.resize(size.width, size.height);
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        ..
                    },
                ..
            } => {
                self.handle_input(code);
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
