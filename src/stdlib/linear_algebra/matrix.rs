use std::ops::{Add, Sub, Mul, Index, IndexMut};
use crate::stdlib::linear_algebra::vector::{Vec2, Vec3};

/// 2x2 Matrix
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Mat2 {
    pub data: [f32; 4],
}

impl Mat2 {
    pub const IDENTITY: Self = Self {
        data: [1.0, 0.0, 0.0, 1.0],
    };
    pub const ZERO: Self = Self {
        data: [0.0, 0.0, 0.0, 0.0],
    };

    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        Self {
            data: [m00, m01, m10, m11],
        }
    }

    pub fn from_cols(col0: Vec2, col1: Vec2) -> Self {
        Self {
            data: [col0.x, col0.y, col1.x, col1.y],
        }
    }

    pub fn from_rows(row0: Vec2, row1: Vec2) -> Self {
        Self {
            data: [row0.x, row1.x, row0.y, row1.y],
        }
    }

    pub fn determinant(self) -> f32 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }

    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < f32::EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Self {
            data: [
                self.data[3] * inv_det,
                -self.data[1] * inv_det,
                -self.data[2] * inv_det,
                self.data[0] * inv_det,
            ],
        })
    }

    pub fn transpose(self) -> Self {
        Self {
            data: [self.data[0], self.data[2], self.data[1], self.data[3]],
        }
    }

    pub fn mul_vec2(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self.data[0] * vec.x + self.data[1] * vec.y,
            y: self.data[2] * vec.x + self.data[3] * vec.y,
        }
    }
}

impl Add for Mat2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
                self.data[3] + rhs.data[3],
            ],
        }
    }
}

impl Sub for Mat2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
                self.data[3] - rhs.data[3],
            ],
        }
    }
}

impl Mul for Mat2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] * rhs.data[0] + self.data[1] * rhs.data[2],
                self.data[0] * rhs.data[1] + self.data[1] * rhs.data[3],
                self.data[2] * rhs.data[0] + self.data[3] * rhs.data[2],
                self.data[2] * rhs.data[1] + self.data[3] * rhs.data[3],
            ],
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.mul_vec2(rhs)
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: [
                self.data[0] * rhs,
                self.data[1] * rhs,
                self.data[2] * rhs,
                self.data[3] * rhs,
            ],
        }
    }
}

impl Index<usize> for Mat2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// 3x3 Matrix
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Mat3 {
    pub data: [f32; 9],
}

impl Mat3 {
    pub const IDENTITY: Self = Self {
        data: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    };
    pub const ZERO: Self = Self {
        data: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32
    ) -> Self {
        Self {
            data: [m00, m01, m02, m10, m11, m12, m20, m21, m22],
        }
    }

    pub fn from_cols(col0: Vec3, col1: Vec3, col2: Vec3) -> Self {
        Self {
            data: [col0.x, col0.y, col0.z, col1.x, col1.y, col1.z, col2.x, col2.y, col2.z],
        }
    }

    pub fn from_rows(row0: Vec3, row1: Vec3, row2: Vec3) -> Self {
        Self {
            data: [row0.x, row1.x, row2.x, row0.y, row1.y, row2.y, row0.z, row1.z, row2.z],
        }
    }

    pub fn translation(x: f32, y: f32) -> Self {
        Self {
            data: [1.0, 0.0, x, 0.0, 1.0, y, 0.0, 0.0, 1.0],
        }
    }

    pub fn scaling(sx: f32, sy: f32) -> Self {
        Self {
            data: [sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn rotation(rad: f32) -> Self {
        let c = rad.cos();
        let s = rad.sin();
        Self {
            data: [c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn determinant(self) -> f32 {
        // Using the rule of Sarrus for 3x3 matrix
        self.data[0] * self.data[4] * self.data[8]
            + self.data[1] * self.data[5] * self.data[6]
            + self.data[2] * self.data[3] * self.data[7]
            - self.data[2] * self.data[4] * self.data[6]
            - self.data[0] * self.data[5] * self.data[7]
            - self.data[1] * self.data[3] * self.data[8]
    }

    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < f32::EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        
        Some(Self {
            data: [
                (self.data[4] * self.data[8] - self.data[5] * self.data[7]) * inv_det,
                (self.data[2] * self.data[7] - self.data[1] * self.data[8]) * inv_det,
                (self.data[1] * self.data[5] - self.data[2] * self.data[4]) * inv_det,
                (self.data[5] * self.data[6] - self.data[3] * self.data[8]) * inv_det,
                (self.data[0] * self.data[8] - self.data[2] * self.data[6]) * inv_det,
                (self.data[2] * self.data[3] - self.data[0] * self.data[5]) * inv_det,
                (self.data[3] * self.data[7] - self.data[4] * self.data[6]) * inv_det,
                (self.data[1] * self.data[6] - self.data[0] * self.data[7]) * inv_det,
                (self.data[0] * self.data[4] - self.data[1] * self.data[3]) * inv_det,
            ],
        })
    }

    pub fn transpose(self) -> Self {
        Self {
            data: [
                self.data[0], self.data[3], self.data[6],
                self.data[1], self.data[4], self.data[7],
                self.data[2], self.data[5], self.data[8],
            ],
        }
    }

    pub fn mul_vec3(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.data[0] * vec.x + self.data[1] * vec.y + self.data[2] * vec.z,
            y: self.data[3] * vec.x + self.data[4] * vec.y + self.data[5] * vec.z,
            z: self.data[6] * vec.x + self.data[7] * vec.y + self.data[8] * vec.z,
        }
    }
}

impl Add for Mat3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
                self.data[3] + rhs.data[3],
                self.data[4] + rhs.data[4],
                self.data[5] + rhs.data[5],
                self.data[6] + rhs.data[6],
                self.data[7] + rhs.data[7],
                self.data[8] + rhs.data[8],
            ],
        }
    }
}

impl Sub for Mat3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
                self.data[3] - rhs.data[3],
                self.data[4] - rhs.data[4],
                self.data[5] - rhs.data[5],
                self.data[6] - rhs.data[6],
                self.data[7] - rhs.data[7],
                self.data[8] - rhs.data[8],
            ],
        }
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] * rhs.data[0] + self.data[1] * rhs.data[3] + self.data[2] * rhs.data[6],
                self.data[0] * rhs.data[1] + self.data[1] * rhs.data[4] + self.data[2] * rhs.data[7],
                self.data[0] * rhs.data[2] + self.data[1] * rhs.data[5] + self.data[2] * rhs.data[8],
                self.data[3] * rhs.data[0] + self.data[4] * rhs.data[3] + self.data[5] * rhs.data[6],
                self.data[3] * rhs.data[1] + self.data[4] * rhs.data[4] + self.data[5] * rhs.data[7],
                self.data[3] * rhs.data[2] + self.data[4] * rhs.data[5] + self.data[5] * rhs.data[8],
                self.data[6] * rhs.data[0] + self.data[7] * rhs.data[3] + self.data[8] * rhs.data[6],
                self.data[6] * rhs.data[1] + self.data[7] * rhs.data[4] + self.data[8] * rhs.data[7],
                self.data[6] * rhs.data[2] + self.data[7] * rhs.data[5] + self.data[8] * rhs.data[8],
            ],
        }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: [
                self.data[0] * rhs,
                self.data[1] * rhs,
                self.data[2] * rhs,
                self.data[3] * rhs,
                self.data[4] * rhs,
                self.data[5] * rhs,
                self.data[6] * rhs,
                self.data[7] * rhs,
                self.data[8] * rhs,
            ],
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// 4x4 Matrix
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Mat4 {
    pub data: [f32; 16],
}

impl Mat4 {
    pub const IDENTITY: Self = Self {
        data: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    };
    pub const ZERO: Self = Self {
        data: [0.0; 16],
    };

    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32
    ) -> Self {
        Self {
            data: [
                m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33,
            ],
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, x,
                0.0, 1.0, 0.0, y,
                0.0, 0.0, 1.0, z,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, c, -s, 0.0,
                0.0, s, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                c, 0.0, s, 0.0,
                0.0, 1.0, 0.0, 0.0,
                -s, 0.0, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                c, -s, 0.0, 0.0,
                s, c, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        let nf = 1.0 / (near - far);
        Self {
            data: [
                f / aspect, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, (far + near) * nf, -1.0,
                0.0, 0.0, (2.0 * far * near) * nf, 0.0,
            ],
        }
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rl = 1.0 / (right - left);
        let tb = 1.0 / (top - bottom);
        let fn_val = 1.0 / (far - near);
        Self {
            data: [
                2.0 * rl, 0.0, 0.0, 0.0,
                0.0, 2.0 * tb, 0.0, 0.0,
                0.0, 0.0, -2.0 * fn_val, 0.0,
                -(right + left) * rl, -(top + bottom) * tb, -(far + near) * fn_val, 1.0,
            ],
        }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let z = (eye - target).normalize();
        let x = up.cross(z).normalize();
        let y = z.cross(x);
        
        Self {
            data: [
                x.x, x.y, x.z, -x.dot(eye),
                y.x, y.y, y.z, -y.dot(eye),
                z.x, z.y, z.z, -z.dot(eye),
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn mul_vec4(self, vec: [f32; 4]) -> [f32; 4] {
        [
            self.data[0] * vec[0] + self.data[1] * vec[1] + self.data[2] * vec[2] + self.data[3] * vec[3],
            self.data[4] * vec[0] + self.data[5] * vec[1] + self.data[6] * vec[2] + self.data[7] * vec[3],
            self.data[8] * vec[0] + self.data[9] * vec[1] + self.data[10] * vec[2] + self.data[11] * vec[3],
            self.data[12] * vec[0] + self.data[13] * vec[1] + self.data[14] * vec[2] + self.data[15] * vec[3],
        ]
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [0.0; 16];
        for i in 0..16 {
            data[i] = self.data[i] + rhs.data[i];
        }
        Self { data }
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [0.0; 16];
        for i in 0..16 {
            data[i] = self.data[i] - rhs.data[i];
        }
        Self { data }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                data[i * 4 + j] = 
                    self.data[i * 4 + 0] * rhs.data[0 * 4 + j] +
                    self.data[i * 4 + 1] * rhs.data[1 * 4 + j] +
                    self.data[i * 4 + 2] * rhs.data[2 * 4 + j] +
                    self.data[i * 4 + 3] * rhs.data[3 * 4 + j];
            }
        }
        Self { data }
    }
}

impl Mul<[f32; 4]> for Mat4 {
    type Output = [f32; 4];

    fn mul(self, rhs: [f32; 4]) -> Self::Output {
        self.mul_vec4(rhs)
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut data = [0.0; 16];
        for i in 0..16 {
            data[i] = self.data[i] * rhs;
        }
        Self { data }
    }
}

impl Index<usize> for Mat4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}