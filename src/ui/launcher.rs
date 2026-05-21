use crate::gpu::{Canvas, GpuContext, Mat3};
use crate::ui::layout::LayoutBox;
use crate::stdlib::{StateRequest, InputManager};
use winit::keyboard::KeyCode;
use winit::event::ElementState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LauncherItemKind {
    Game,
    Example,
}

#[derive(Clone)]
pub struct LauncherItem {
    pub name: String,
    pub kind: LauncherItemKind,
}

pub struct LauncherState {
    pub items: Vec<LauncherItem>,
    pub search_text: String,
    pub selected_index: usize,
    pub cursor_blink: f64,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            items: vec![
                LauncherItem {
                    name: "Snake Game".to_string(),
                    kind: LauncherItemKind::Game,
                },
                LauncherItem {
                    name: "Key Indicator".to_string(),
                    kind: LauncherItemKind::Game,
                },
                LauncherItem {
                    name: "Breakouts".to_string(),
                    kind: LauncherItemKind::Game,
                },
                LauncherItem {
                    name: "Ping Pong".to_string(),
                    kind: LauncherItemKind::Game,
                },
                LauncherItem {
                    name: "Linear Algebra Demo".to_string(),
                    kind: LauncherItemKind::Example,
                },
            ],
            search_text: String::new(),
            selected_index: 0,
            cursor_blink: 0.0,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode, state: ElementState, gpu: &GpuContext) -> Option<StateRequest> {
        if state != ElementState::Pressed {
            return None;
        }

        match key {
            KeyCode::Enter => {
                let filtered = self.filtered_items();
                if let Some((idx, _)) = filtered.get(self.selected_index) {
                    match *idx {
                        0 => return Some(StateRequest::Push(Box::new(crate::game::SnakeState::new(gpu)))),
                        1 => return Some(StateRequest::Push(Box::new(crate::game::KeyIndicatorState::new(gpu)))),
                        2 => return Some(StateRequest::Push(Box::new(crate::game::breakouts::BreakoutsState::new(gpu)))),
                        3 => return Some(StateRequest::Push(Box::new(crate::game::pingpong::PingPongState::new(gpu)))),
                        4 => return Some(StateRequest::Push(Box::new(crate::game::LinearAlgebraState::new(gpu)))),
                        _ => {}
                    }
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
                item.name.to_lowercase().contains(&query)
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
        for (i, (_, _)) in filtered.iter().enumerate().take(8) {
            let y_top = item_y_start + (i as f32) * (item_h + item_gap);
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
    }

    pub fn backspace(&mut self) {
        self.search_text.pop();
        self.selected_index = 0;
    }

    pub fn move_selection(&mut self, delta: i32) {
        let filtered = self.filtered_items();
        if filtered.is_empty() {
            return;
        }
        let max = filtered.len() - 1;
        if delta > 0 {
            self.selected_index = (self.selected_index + 1).min(max);
        } else {
            self.selected_index = self.selected_index.saturating_sub(1);
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.cursor_blink += delta;
    }

    pub fn handle_mouse_click(&mut self, mouse_pos: [f32; 2], sw: f32, sh: f32, gpu: &GpuContext) -> Option<StateRequest> {
        if let Some(i) = self.get_item_at(mouse_pos, sw, sh) {
            self.selected_index = i;
            let filtered = self.filtered_items();
            if let Some((idx, _)) = filtered.get(self.selected_index) {
                match *idx {
                    0 => return Some(StateRequest::Push(Box::new(crate::game::SnakeState::new(gpu)))),
                    1 => return Some(StateRequest::Push(Box::new(crate::game::KeyIndicatorState::new(gpu)))),
                    2 => return Some(StateRequest::Push(Box::new(crate::game::breakouts::BreakoutsState::new(gpu)))),
                    3 => return Some(StateRequest::Push(Box::new(crate::game::pingpong::PingPongState::new(gpu)))),
                    4 => return Some(StateRequest::Push(Box::new(crate::game::LinearAlgebraState::new(gpu)))),
                    _ => {}
                }
            }
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

        for (i, (_, item)) in filtered.iter().enumerate().take(8) {
            let y = item_y_start + (i as f32) * (item_h + item_gap);
            let is_selected = i == launcher.selected_index;

            self.canvas.draw_rectangle(item_x, y, item_w, item_h, if is_selected { [0.3, 0.2, 0.5, 1.0] } else { [0.2, 0.2, 0.35, 1.0] });

            let name_color = if is_selected { [0.85, 0.85, 0.95, 1.0] } else { [0.7, 0.7, 0.8, 1.0] };
            self.canvas.draw_text(gpu, &item.name, item_x + 12.0, y + 8.0, 20.0, name_color);

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
        }

        let hint_y = panel.size.y - 24.0;
        self.canvas.draw_text(gpu, "Arrow keys: navigate | Enter: select | Esc: clear", pad, hint_y, 14.0, [0.5, 0.5, 0.7, 1.0]);

        self.canvas.pop_transform();

        self.canvas.end_drawing(gpu, encoder, view, [0.05, 0.05, 0.1, 1.0]);
    }
}