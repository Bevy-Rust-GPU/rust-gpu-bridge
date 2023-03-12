//! WGSL `saturate()`

use crate::glam::{Vec2, Vec3, Vec4};

/// Equivalent of the WGSL `saturate()` function.
///
/// Clamps a value to the 0.0..=1.0 range
pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Clamp for f32 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f32::clamp(self, min, max)
    }
}

impl Clamp for Vec2 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Vec2::clamp(self, min, max)
    }
}

impl Clamp for Vec3 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Vec3::clamp(self, min, max)
    }
}

impl Clamp for Vec4 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Vec4::clamp(self, min, max)
    }
}
