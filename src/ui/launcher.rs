use crate::gpu::{Canvas, GpuContext};

pub struct LauncherState {
    pub items: Vec<LauncherItem>,
    pub search_text: String,
    pub selected_index: usize,
    pub cursor_blink: f64,
}

#[derive(Clone)]
pub struct LauncherItem {
    pub name: String,
    pub icon: char,
    pub description: String,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            items: vec![
                LauncherItem {
                    name: "Snake Game".to_string(),
                    icon: '\u{25B6}',
                    description: "Classic snake game".to_string(),
                },
                LauncherItem {
                    name: "Triangle Demo".to_string(),
                    icon: '\u{25B2}',
                    description: "Basic WebGPU triangle".to_string(),
                },
                LauncherItem {
                    name: "Settings".to_string(),
                    icon: '\u{2699}',
                    description: "Application settings".to_string(),
                },
                LauncherItem {
                    name: "About".to_string(),
                    icon: '\u{2139}',
                    description: "About this application".to_string(),
                },
                LauncherItem {
                    name: "Exit".to_string(),
                    icon: '\u{2716}',
                    description: "Quit application".to_string(),
                },
            ],
            search_text: String::new(),
            selected_index: 0,
            cursor_blink: 0.0,
        }
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
                    || item.description.to_lowercase().contains(&query)
            })
            .collect()
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

    pub fn render(&mut self, gpu: &GpuContext, view: &wgpu::TextureView, launcher: &LauncherState) {
        let sw = gpu.surface_config.width as f32;
        let sh = gpu.surface_config.height as f32;

        let panel_w = sw * 0.55;
        let panel_h = sh * 0.65;
        let panel_x = (sw - panel_w) / 2.0;
        let panel_y = (sh - panel_h) / 2.0;
        let pad = 16.0;

        // Background
        self.canvas.draw_rectangle(0.0, 0.0, sw, sh, [0.05, 0.05, 0.1, 1.0]);
        
        // Panel
        self.canvas.draw_rectangle(panel_x, panel_y, panel_w, panel_h, [0.12, 0.12, 0.22, 1.0]);

        let search_h = 36.0;
        let search_y = panel_y + pad;
        let search_x = panel_x + pad;
        let search_w = panel_w - pad * 2.0;

        // Search Bar
        self.canvas.draw_rectangle(search_x, search_y, search_w, search_h, [0.18, 0.18, 0.32, 1.0]);

        // Text in search bar
        let prompt = "> ";
        self.canvas.draw_text(prompt, search_x + 8.0, search_y + 6.0, 20.0, [0.2, 1.0, 0.3, 1.0]);
        
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
        self.canvas.draw_text(&search_text, search_x + 8.0 + prompt_w + 4.0, search_y + 6.0, 20.0, search_color);

        if !launcher.search_text.is_empty() && launcher.cursor_blink.sin() > 0.0 {
            let cursor_x = search_x + 8.0 + prompt_w + 4.0 + (launcher.search_text.len() as f32 * 12.0);
            self.canvas.draw_rectangle(cursor_x, search_y + 8.0, 2.0, search_h - 16.0, [0.85, 0.85, 0.95, 1.0]);
        }

        let filtered = launcher.filtered_items();
        let item_h = 48.0;
        let item_gap = 4.0;
        let item_y_start = search_y + search_h + 12.0;
        let item_x = panel_x + pad;
        let item_w = panel_w - pad * 2.0;

        for (i, (_, item)) in filtered.iter().enumerate().take(8) {
            let y = item_y_start + (i as f32) * (item_h + item_gap);
            let is_selected = i == launcher.selected_index;

            self.canvas.draw_rectangle(item_x, y, item_w, item_h, if is_selected { [0.3, 0.2, 0.5, 1.0] } else { [0.2, 0.2, 0.35, 1.0] });

            let icon_color = if is_selected { [1.0, 0.85, 0.2, 1.0] } else { [0.2, 1.0, 0.3, 1.0] };
            self.canvas.draw_text(&item.icon.to_string(), item_x + 12.0, y + 10.0, 24.0, icon_color);

            let name_color = if is_selected { [0.85, 0.85, 0.95, 1.0] } else { [0.5, 0.5, 0.7, 1.0] };
            self.canvas.draw_text(&item.name, item_x + 44.0, y + 8.0, 20.0, name_color);

            self.canvas.draw_text(&item.description, item_x + 44.0, y + 28.0, 14.0, [0.5, 0.5, 0.7, 1.0]);
        }

        let hint_y = panel_y + panel_h - 24.0;
        self.canvas.draw_text("Arrow keys: navigate | Enter: select | Esc: clear", panel_x + pad, hint_y, 14.0, [0.5, 0.5, 0.7, 1.0]);

        self.canvas.end_drawing(gpu, view, [0.05, 0.05, 0.1, 1.0]);
    }
}
