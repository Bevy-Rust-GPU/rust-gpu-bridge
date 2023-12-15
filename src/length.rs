//! Shader `length()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns the length of self.
///
/// Equivalent of the `length()` function.
pub trait Length {
    type Output;

    fn length(self) -> Self::Output;
}

impl Length for f32 {
    type Output = f32;

    fn length(self) -> Self::Output {
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

impl Length for f64 {
    type Output = f64;

    fn length(self) -> Self::Output {
        #[cfg(feature = "glam")]
        {
            f64::abs(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::abs(self)
        }
    }
}

impl Length for Vec2 {
    type Output = f32;

    fn length(self) -> Self::Output {
        Vec2::length(self)
    }
}

impl Length for Vec3 {
    type Output = f32;

    fn length(self) -> Self::Output {
        Vec3::length(self)
    }
}

impl Length for Vec4 {
    type Output = f32;

    fn length(self) -> Self::Output {
        Vec4::length(self)
    }
}

impl Length for DVec2 {
    type Output = f64;

    fn length(self) -> Self::Output {
        DVec2::length(self)
    }
}

impl Length for DVec3 {
    type Output = f64;

    fn length(self) -> Self::Output {
        DVec3::length(self)
    }
}

impl Length for DVec4 {
    type Output = f64;

    fn length(self) -> Self::Output {
        DVec4::length(self)
    }
}
