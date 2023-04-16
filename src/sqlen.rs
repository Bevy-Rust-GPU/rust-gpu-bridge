//! Shader `sqlen()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns the length of self.
///
/// Equivalent of the `length()` function.
pub trait SquareLength {
    fn sqlen(self) -> f32;
}

impl SquareLength for f32 {
    fn sqlen(self) -> f32 {
        #[cfg(feature = "glam")]
        {
            f32::abs(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::abs(self)
        }
    }
}

impl SquareLength for Vec2 {
    fn sqlen(self) -> f32 {
        Vec2::length_squared(self)
    }
}

impl SquareLength for Vec3 {
    fn sqlen(self) -> f32 {
        Vec3::length_squared(self)
    }
}

impl SquareLength for Vec4 {
    fn sqlen(self) -> f32 {
        Vec4::length_squared(self)
    }
}

