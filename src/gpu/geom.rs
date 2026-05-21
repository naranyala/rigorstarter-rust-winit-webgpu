use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self { x: self.x / len, y: self.y / len }
        } else {
            Self::ZERO
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        Self {
            r: a.r + (b.r - a.r) * t,
            g: a.g + (b.g - a.g) * t,
            b: a.b + (b.b - a.b) * t,
            a: a.a + (b.a - a.a) * t,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(w, h),
        }
    }

    pub fn left(&self) -> f32 { self.pos.x }
    pub fn right(&self) -> f32 { self.pos.x + self.size.x }
    pub fn top(&self) -> f32 { self.pos.y }
    pub fn bottom(&self) -> f32 { self.pos.y + self.size.y }
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.pos.x + self.size.x / 2.0, self.pos.y + self.size.y / 2.0)
    }
}

/// Simple 3x3 Matrix for 2D Transformations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub data: [f32; 9],
}

impl Mat3 {
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn translation(x: f32, y: f32) -> Self {
        let mut m = Self::identity();
        m.data[2] = x;
        m.data[5] = y;
        m
    }

    pub fn scaling(sx: f32, sy: f32) -> Self {
        let mut m = Self::identity();
        m.data[0] = sx;
        m.data[4] = sy;
        m
    }

    pub fn rotation(rad: f32) -> Self {
        let mut m = Self::identity();
        let sin = rad.sin();
        let cos = rad.cos();
        m.data[0] = cos;
        m.data[1] = sin;
        m.data[3] = -sin;
        m.data[4] = cos;
        m
    }

    pub fn multiply(&self, rhs: &Self) -> Self {
        let mut res = [0.0; 9];
        for i in 0..3 {
            for j in 0..3 {
                res[i * 3 + j] = 
                    self.data[i * 3 + 0] * rhs.data[0 * 3 + j] +
                    self.data[i * 3 + 1] * rhs.data[1 * 3 + j] +
                    self.data[i * 3 + 2] * rhs.data[2 * 3 + j];
            }
        }
        Self { data: res }
    }

    pub fn transform_point(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            self.data[0] * p.x + self.data[1] * p.y + self.data[2],
            self.data[3] * p.x + self.data[4] * p.y + self.data[5],
        )
    }
}
