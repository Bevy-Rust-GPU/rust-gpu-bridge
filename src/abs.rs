//! WGSL `saturate()`

use crate::glam::{Vec2, Vec3, Vec4};

/// Equivalent of the WGSL `saturate()` function.
///
/// Clamps a value to the 0.0..=1.0 range
pub trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self {
        f32::abs(self)
    }
}

impl Abs for Vec2 {
    fn abs(self) -> Self {
        Vec2::abs(self)
    }
}

impl Abs for Vec3 {
    fn abs(self) -> Self {
        Vec3::abs(self)
    }
}

impl Abs for Vec4 {
    fn abs(self) -> Self {
        Vec4::abs(self)
    }
}
