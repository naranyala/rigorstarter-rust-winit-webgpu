use crate::gpu::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

pub struct LayoutBox {
    pub pos: Vec2,
    pub size: Vec2,
}

impl LayoutBox {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(w, h),
        }
    }

    pub fn center_of(&self, width: f32, height: f32) -> Self {
        Self {
            pos: Vec2::new((width - self.size.x) / 2.0, (height - self.size.y) / 2.0),
            size: self.size,
        }
    }

    pub fn offset(&self, x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(self.pos.x + x, self.pos.y + y),
            size: self.size,
        }
    }

    pub fn align_x(&self, align: Alignment, container_w: f32) -> f32 {
        match align {
            Alignment::Start => self.pos.x,
            Alignment::Center => (container_w - self.size.x) / 2.0,
            Alignment::End => container_w - self.size.x,
        }
    }

    pub fn align_y(&self, align: Alignment, container_h: f32) -> f32 {
        match align {
            Alignment::Start => self.pos.y,
            Alignment::Center => (container_h - self.size.y) / 2.0,
            Alignment::End => container_h - self.size.y,
        }
    }
}
