use crate::gpu::{Canvas, GpuContext, Mat3};
use crate::ui::layout::LayoutBox;
use crate::stdlib::{State, StateRequest};
use winit::keyboard::KeyCode;
use winit::event::ElementState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LauncherItemKind {
    Game,
    Example,
}

pub struct LauncherItem {
    pub name: String,
    pub description: String,
    pub kind: LauncherItemKind,
    pub creator: fn(&GpuContext) -> Box<dyn State>,
}

pub struct LauncherState {
    pub items: Vec<LauncherItem>,
    pub search_text: String,
    pub selected_index: usize,
    pub cursor_blink: f64,
    pub scroll_offset: usize,
    pub current_visible_count: usize,
}

impl LauncherState {
    pub fn new() -> Self {
        let items = vec![
            LauncherItem {
                name: "Snake Game".to_string(),
                description: "Classic grid-based snake action.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(crate::game::SnakeState::new(gpu)),
            },
            LauncherItem {
                name: "Breakouts".to_string(),
                description: "Destroy bricks with a bouncing ball.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(crate::game::breakouts::BreakoutsState::new(gpu)),
            },
            LauncherItem {
                name: "Ping Pong".to_string(),
                description: "Fast-paced table tennis simulator.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(crate::game::pingpong::PingPongState::new(gpu)),
            },
            LauncherItem {
                name: "Linear Algebra Demo".to_string(),
                description: "3D rotating cube and matrix math.".to_string(),
                kind: LauncherItemKind::Example,
                creator: |gpu| Box::new(crate::examples::cube_demo::CubeDemoState::new(gpu)),
            },
            LauncherItem {
                name: "Key Press History".to_string(),
                description: "Visualizer for keyboard input events.".to_string(),
                kind: LauncherItemKind::Example,
                creator: |gpu| Box::new(crate::examples::keypress_demo::KeyPressDemoState::new(gpu)),
            },
            LauncherItem {
                name: "Trig Wave Visualizer".to_string(),
                description: "Procedural sine wave animation.".to_string(),
                kind: LauncherItemKind::Example,
                creator: |gpu| Box::new(crate::examples::trig_wave_demo::TrigWaveDemoState::new(gpu)),
            },
            LauncherItem {
                name: "Particle Flow Field".to_string(),
                description: "Vector field integration visualizer.".to_string(),
                kind: LauncherItemKind::Example,
                creator: |gpu| Box::new(crate::examples::flow_field_demo::FlowFieldDemoState::new(gpu)),
            },
            LauncherItem {
                name: "Bézier Curve Editor".to_string(),
                description: "Recursive interpolation demo.".to_string(),
                kind: LauncherItemKind::Example,
                creator: |gpu| Box::new(crate::examples::bezier_demo::BezierDemoState::new(gpu)),
            },
            LauncherItem {
                name: "Asteroids".to_string(),
                description: "Space combat and physics demo.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(PlaceholderState::new("Asteroids", gpu)),
            },
            LauncherItem {
                name: "Flappy Bird".to_string(),
                description: "Gravity and impulse physics example.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(PlaceholderState::new("Flappy Bird", gpu)),
            },
            LauncherItem {
                name: "Space Invaders".to_string(),
                description: "Classic alien invasion shooter.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(PlaceholderState::new("Space Invaders", gpu)),
            },
            LauncherItem {
                name: "Tetris".to_string(),
                description: "Classic block-fitting puzzle game.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(PlaceholderState::new("Tetris", gpu)),
            },
            LauncherItem {
                name: "Pacman".to_string(),
                description: "Maze navigation and AI demo.".to_string(),
                kind: LauncherItemKind::Game,
                creator: |gpu| Box::new(PlaceholderState::new("Pacman", gpu)),
            },
        ];

        Self {
            items,
            search_text: String::new(),
            selected_index: 0,
            cursor_blink: 0.0,
            scroll_offset: 0,
            current_visible_count: 8,
        }
    }

    pub fn update_layout(&mut self, sw: f32, sh: f32) {
        let panel_h = sh * 0.65;
        let pad = 16.0;
        let search_h = 36.0;
        let item_h = 48.0;
        let item_gap = 4.0;
        let footer_h = 24.0;
        let item_y_start = pad + search_h + 12.0;

        let available_h = panel_h - item_y_start - footer_h - pad;
        self.current_visible_count = (available_h / (item_h + item_gap)).floor() as usize;
        self.current_visible_count = self.current_visible_count.max(1);
    }

    pub fn handle_input(&mut self, key: KeyCode, state: ElementState, gpu: &GpuContext) -> Option<StateRequest> {
        if state != ElementState::Pressed {
            return None;
        }

        match key {
            KeyCode::Enter => {
                let filtered = self.filtered_items();
                if let Some((_, item)) = filtered.get(self.selected_index) {
                    return Some(StateRequest::Push((item).creator(gpu)));
                }
            }
            KeyCode::ArrowUp => {
                self.move_selection(-1);
            }
            KeyCode::ArrowDown => {
                self.move_selection(1);
            }
            KeyCode::Backspace => {
                self.backspace();
            }
            KeyCode::Escape => {
                self.search_text.clear();
                self.selected_index = 0;
                self.scroll_offset = 0;
            }
            _ => {}
        }
        None
    }

    pub fn filtered_items(&self) -> Vec<(usize, &LauncherItem)> {
        if self.search_text.is_empty() {
            return self.items.iter().enumerate().map(|(i, item)| (i, item)).collect();
        }
        let query = self.search_text.to_lowercase();
        self.items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item.name.to_lowercase().contains(&query) || item.description.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn get_item_at(&self, mouse_pos: [f32; 2], sw: f32, sh: f32) -> Option<usize> {
        let panel_w = sw * 0.55;
        let panel_h = sh * 0.65;
        let panel_x = (sw - panel_w) / 2.0;
        let panel_y = (sh - panel_h) / 2.0;
        let pad = 16.0;
        let search_h = 36.0;
        let item_h = 48.0;
        let item_gap = 4.0;
        let item_y_start = pad + search_h + 12.0;

        let mx = mouse_pos[0];
        let my = mouse_pos[1];

        if mx < panel_x || mx > panel_x + panel_w || my < panel_y || my > panel_y + panel_h {
            return None;
        }

        let rel_y = my - panel_y;
        if rel_y < item_y_start {
            return None;
        }

        let filtered = self.filtered_items();
        for i in self.scroll_offset..(self.scroll_offset + self.current_visible_count).min(filtered.len()) {
            let i_in_view = i - self.scroll_offset;
            let y_top = item_y_start + (i_in_view as f32) * (item_h + item_gap);
            let y_bottom = y_top + item_h;

            if rel_y >= y_top && rel_y <= y_bottom {
                return Some(i);
            }
        }
        None
    }

    pub fn input_char(&mut self, c: char) {
        if c.is_control() {
            return;
        }
        self.search_text.push(c);
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn backspace(&mut self) {
        self.search_text.pop();
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn move_selection(&mut self, delta: i32) {
        let filtered = self.filtered_items();
        if filtered.is_empty() {
            return;
        }
        let max = filtered.len() - 1;
        if delta > 0 {
            self.selected_index = (self.selected_index + 1).min(max);
        } else if delta < 0 {
            self.selected_index = self.selected_index.saturating_sub(1);
        }

        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + self.current_visible_count {
            self.scroll_offset = self.selected_index - self.current_visible_count + 1;
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.cursor_blink += delta;
    }

    pub fn handle_mouse_wheel(&mut self, _delta_x: f32, delta_y: f32) {
        let filtered = self.filtered_items();
        if filtered.is_empty() {
            return;
        }

        if delta_y > 0.0 {
            self.scroll_offset = self.scroll_offset.saturating_sub(1);
        } else if delta_y < 0.0 {
            let max_offset = filtered.len().saturating_sub(self.current_visible_count);
            if self.scroll_offset < max_offset {
                self.scroll_offset += 1;
            }
        }
    }

    pub fn handle_mouse_click(&mut self, mouse_pos: [f32; 2], sw: f32, sh: f32, gpu: &GpuContext) -> Option<StateRequest> {
        if let Some(i) = self.get_item_at(mouse_pos, sw, sh) {
            self.selected_index = i;
            let filtered = self.filtered_items();
            if let Some((_, item)) = filtered.get(self.selected_index) {
                return Some(StateRequest::Push((item).creator(gpu)));
            }
        }
        None
    }
}

pub struct PlaceholderState {
    name: String,
    canvas: Canvas,
}

impl PlaceholderState {
    pub fn new(name: &str, gpu: &GpuContext) -> Self {
        Self {
            name: name.to_string(),
            canvas: Canvas::new(gpu),
        }
    }
}

impl State for PlaceholderState {
    fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let sw = gpu.surface_config.width as f32;
        let sh = gpu.surface_config.height as f32;
        let clear_color = [0.1, 0.1, 0.15, 1.0];
        
        self.canvas.draw_rectangle(0.0, 0.0, sw, sh, clear_color);
        self.canvas.draw_text(gpu, &format!("{} is under construction!", self.name), sw / 2.0 - 100.0, sh / 2.0, 24.0, [1.0, 1.0, 1.0, 1.0]);
        self.canvas.draw_text(gpu, "Press ESC to return to launcher", sw / 2.0 - 120.0, sh / 2.0 + 40.0, 18.0, [0.6, 0.6, 0.6, 1.0]);
        self.canvas.end_drawing(gpu, encoder, view, clear_color);
    }

    fn handle_input(&mut self, key: KeyCode, state: ElementState) -> Option<StateRequest> {
        if state == ElementState::Pressed && key == KeyCode::Escape {
            return Some(StateRequest::Pop);
        }
        None
    }
}

pub struct LauncherRenderer {
    canvas: Canvas,
}

impl LauncherRenderer {
    pub fn new(gpu: &GpuContext) -> Self {
        Self {
            canvas: Canvas::new(gpu),
        }
    }

    pub fn render(&mut self, gpu: &GpuContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, launcher: &LauncherState) {
        let sw = gpu.surface_config.width as f32;
        let sh = gpu.surface_config.height as f32;

        let panel_w = sw * 0.55;
        let panel_h = sh * 0.65;
        
        let panel = LayoutBox::new(
            (sw - panel_w) / 2.0,
            (sh - panel_h) / 2.0,
            panel_w,
            panel_h,
        );
        let pad = 16.0;

        // Background
        self.canvas.draw_rectangle(0.0, 0.0, sw, sh, [0.05, 0.05, 0.1, 1.0]);
        
        // Panel
        self.canvas.draw_rectangle(panel.pos.x, panel.pos.y, panel.size.x, panel.size.y, [0.12, 0.12, 0.22, 1.0]);

        // Switch to panel coordinates
        self.canvas.push_transform(Mat3::translation(panel.pos.x, panel.pos.y));

        let search_h = 36.0;
        let search_y = pad;
        let search_x = pad;
        let search_w = panel.size.x - pad * 2.0;

        // Search Bar
        self.canvas.draw_rectangle(search_x, search_y, search_w, search_h, [0.18, 0.18, 0.32, 1.0]);

        // Text in search bar
        let prompt = "> ";
        self.canvas.draw_text(gpu, prompt, search_x + 8.0, search_y + 6.0, 20.0, [0.2, 1.0, 0.3, 1.0]);
        
        let prompt_w = prompt.len() as f32 * 12.0;
        let search_text = if launcher.search_text.is_empty() {
            "Type to search...".to_string()
        } else {
            launcher.search_text.clone()
        };
        let search_color = if launcher.search_text.is_empty() {
            [0.5, 0.5, 0.7, 1.0]
        } else {
            [0.85, 0.85, 0.95, 1.0]
        };
        self.canvas.draw_text(gpu, &search_text, search_x + 8.0 + prompt_w + 4.0, search_y + 6.0, 20.0, search_color);

        if !launcher.search_text.is_empty() && launcher.cursor_blink.sin() > 0.0 {
            let cursor_x = search_x + 8.0 + prompt_w + 4.0 + (launcher.search_text.len() as f32 * 12.0);
            self.canvas.draw_rectangle(cursor_x, search_y + 8.0, 2.0, search_h - 16.0, [0.85, 0.85, 0.95, 1.0]);
        }

        let filtered = launcher.filtered_items();
        let item_h = 48.0;
        let item_gap = 4.0;
        let item_y_start = search_y + search_h + 12.0;
        let item_x = pad;
        let item_w = panel.size.x - pad * 2.0;

        let mut count = 0;
        for (i, (_, item)) in filtered.iter().enumerate() {
            if i < launcher.scroll_offset { continue; }
            if count >= launcher.current_visible_count { break; }

            let y = item_y_start + (count as f32) * (item_h + item_gap);
            let is_selected = i == launcher.selected_index;

            self.canvas.draw_rectangle(item_x, y, item_w, item_h, if is_selected { [0.3, 0.2, 0.5, 1.0] } else { [0.2, 0.2, 0.35, 1.0] });

            let name_color = if is_selected { [0.85, 0.85, 0.95, 1.0] } else { [0.7, 0.7, 0.8, 1.0] };
            self.canvas.draw_text(gpu, &item.name, item_x + 12.0, y + 8.0, 20.0, name_color);
            
            // Draw description below the name
            let desc_color = if is_selected { [0.6, 0.6, 0.8, 1.0] } else { [0.4, 0.4, 0.6, 1.0] };
            self.canvas.draw_text(gpu, &item.description, item_x + 12.0, y + 24.0, 14.0, desc_color);

            // Badge
            let badge_text = match item.kind {
                LauncherItemKind::Game => "[Game]",
                LauncherItemKind::Example => "[Example]",
            };
            let badge_color = match item.kind {
                LauncherItemKind::Game => [0.2, 0.8, 0.2, 1.0],
                LauncherItemKind::Example => [0.8, 0.8, 0.2, 1.0],
            };
            let badge_x = item_w - 80.0;
            self.canvas.draw_text(gpu, badge_text, item_x + badge_x, y + 8.0, 16.0, badge_color);
            
            count += 1;
        }

        let footer_h = 24.0;
        let hint_y = panel.size.y - footer_h - pad;
        self.canvas.draw_text(gpu, "Arrow keys: navigate | Enter: select | Esc: clear", pad, hint_y, 14.0, [0.5, 0.5, 0.7, 1.0]);

        self.canvas.pop_transform();

        self.canvas.end_drawing(gpu, encoder, view, [0.05, 0.05, 0.1, 1.0]);
    }
}
