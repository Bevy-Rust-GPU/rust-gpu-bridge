//! WGSL `reflect()`

use crate::glam::{Vec2, Vec3, Vec4};

/// Equivalent of the WGSL `reflect()` function.
///
/// Reflects a vector about a provided normal.
pub trait Reflect {
    fn reflect(self, normal: Self) -> Self;
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

