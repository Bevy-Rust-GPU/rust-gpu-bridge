//! Shader `reflect()`

use crate::glam::{Vec2, Vec3, Vec4};

/// Reflects self about a provided normal.
///
/// Equivalent of the `reflect()` function.
pub trait Reflect {
    fn reflect(self, normal: Self) -> Self;
}

impl Reflect for f32 {
    fn reflect(self, normal: Self) -> Self {
        -2.0 * (self * normal) * normal + self
    }
}

impl Reflect for Vec2 {
    fn reflect(self, normal: Self) -> Self {
        -2.0 * (self.dot(normal)) * normal + self
    }
}

impl Reflect for Vec3 {
    fn reflect(self, normal: Self) -> Self {
        -2.0 * (self.dot(normal)) * normal + self
    }
}

impl Reflect for Vec4 {
    fn reflect(self, normal: Self) -> Self {
        -2.0 * (self.dot(normal)) * normal + self
    }
}
