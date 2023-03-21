//! Shader `length()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns the length of self.
///
/// Equivalent of the `length()` function.
pub trait Length {
    fn length(self) -> f32;
}

impl Length for f32 {
    fn length(self) -> f32 {
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

impl Length for Vec2 {
    fn length(self) -> f32 {
        Vec2::length(self)
    }
}

impl Length for Vec3 {
    fn length(self) -> f32 {
        Vec3::length(self)
    }
}

impl Length for Vec4 {
    fn length(self) -> f32 {
        Vec4::length(self)
    }
}

