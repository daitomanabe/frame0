use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalized(self) -> Self {
        let length = self.length();
        if length <= f32::EPSILON {
            Self::ZERO
        } else {
            self / length
        }
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorRgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorRgba {
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_prime = (h.rem_euclid(360.0)) / 60.0;
        let x = c * (1.0 - (h_prime.rem_euclid(2.0) - 1.0).abs());
        let (r1, g1, b1) = match h_prime as u32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        let m = l - c / 2.0;
        Self::rgb(r1 + m, g1 + m, b1 + m)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            origin: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    pub fn center(self) -> Vec2 {
        self.origin + self.size * 0.5
    }

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.origin.x
            && point.y >= self.origin.y
            && point.x <= self.origin.x + self.size.x
            && point.y <= self.origin.y + self.size.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub translation: Vec2,
    pub rotation_rad: f32,
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation_rad: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

impl Transform2D {
    pub fn apply(self, point: Vec2) -> Vec2 {
        let scaled = Vec2::new(point.x * self.scale.x, point.y * self.scale.y);
        let (sin, cos) = self.rotation_rad.sin_cos();
        Vec2::new(
            scaled.x * cos - scaled.y * sin,
            scaled.x * sin + scaled.y * cos,
        ) + self.translation
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Polyline {
    pub points: Vec<Vec2>,
    pub closed: bool,
}

impl Polyline {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self {
            points,
            closed: false,
        }
    }

    pub fn closed(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn length(&self) -> f32 {
        let mut total = self
            .points
            .windows(2)
            .map(|pair| (pair[1] - pair[0]).length())
            .sum();
        if self.closed && self.points.len() > 2 {
            total += (self.points[0] - *self.points.last().unwrap()).length();
        }
        total
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub color: ColorRgba,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn quad(rect: Rect, color: ColorRgba) -> Self {
        let z = 0.0;
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let x = rect.origin.x;
        let y = rect.origin.y;
        let w = rect.size.x;
        let h = rect.size.y;
        Self {
            vertices: vec![
                Vertex {
                    position: Vec3::new(x, y, z),
                    normal,
                    uv: Vec2::new(0.0, 0.0),
                    color,
                },
                Vertex {
                    position: Vec3::new(x + w, y, z),
                    normal,
                    uv: Vec2::new(1.0, 0.0),
                    color,
                },
                Vertex {
                    position: Vec3::new(x + w, y + h, z),
                    normal,
                    uv: Vec2::new(1.0, 1.0),
                    color,
                },
                Vertex {
                    position: Vec3::new(x, y + h, z),
                    normal,
                    uv: Vec2::new(0.0, 1.0),
                    color,
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Easing {
    Linear,
    InQuad,
    OutQuad,
    InOutCubic,
    SineInOut,
}

impl Easing {
    pub fn sample(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::InQuad => t * t,
            Self::OutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Self::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Self::SineInOut => -(PI * t).cos() / 2.0 + 0.5,
        }
    }
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn value_noise_2d(x: f32, y: f32, seed: u32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - xi as f32;
    let yf = y - yi as f32;
    let a = hash_to_unit(xi, yi, seed);
    let b = hash_to_unit(xi + 1, yi, seed);
    let c = hash_to_unit(xi, yi + 1, seed);
    let d = hash_to_unit(xi + 1, yi + 1, seed);
    let u = smoothstep(0.0, 1.0, xf);
    let v = smoothstep(0.0, 1.0, yf);
    let x1 = a + (b - a) * u;
    let x2 = c + (d - c) * u;
    x1 + (x2 - x1) * v
}

fn hash_to_unit(x: i32, y: i32, seed: u32) -> f32 {
    let mut n = x as u32;
    n = n.wrapping_mul(0x27d4_eb2d);
    n ^= (y as u32).wrapping_mul(0x1656_67b1);
    n ^= seed.wrapping_mul(0x9e37_79b9);
    n ^= n >> 15;
    n = n.wrapping_mul(0x85eb_ca6b);
    n ^= n >> 13;
    n as f32 / u32::MAX as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_math_is_stable() {
        let a = Vec2::new(3.0, 4.0);
        assert_eq!(a.length(), 5.0);
        assert_eq!(a.normalized(), Vec2::new(0.6, 0.8));
    }

    #[test]
    fn quad_mesh_has_expected_indices() {
        let mesh = Mesh::quad(Rect::new(0.0, 0.0, 10.0, 20.0), ColorRgba::WHITE);
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices, vec![0, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn easing_is_clamped() {
        assert_eq!(Easing::Linear.sample(-1.0), 0.0);
        assert_eq!(Easing::Linear.sample(2.0), 1.0);
        assert!(Easing::InOutCubic.sample(0.5) > 0.49);
    }

    #[test]
    fn noise_is_deterministic() {
        assert_eq!(value_noise_2d(1.25, 2.5, 42), value_noise_2d(1.25, 2.5, 42));
    }
}
