//! Shader `clamp()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Clamps a value to the provided range.
///
/// Equivalent of the `clamp()` function.
pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Clamp for f32 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f32::clamp(self, min, max)
    }
}

impl Clamp for f64 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f64::clamp(self, min, max)
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

impl Clamp for DVec2 {
    fn clamp(self, min: Self, max: Self) -> Self {
        DVec2::clamp(self, min, max)
    }
}

impl Clamp for DVec3 {
    fn clamp(self, min: Self, max: Self) -> Self {
        DVec3::clamp(self, min, max)
    }
}

impl Clamp for DVec4 {
    fn clamp(self, min: Self, max: Self) -> Self {
        DVec4::clamp(self, min, max)
    }
}
