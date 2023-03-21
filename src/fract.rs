//! Shader `fract()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns the fractional part of self.
///
/// Equivalent of the `fract()` function.
pub trait Fract {
    fn fract(self) -> Self;
}

impl Fract for f32 {
    fn fract(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::fract(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::fract(self)
        }
    }
}

impl Fract for Vec2 {
    fn fract(self) -> Self {
        Vec2::new(Fract::fract(self.x), Fract::fract(self.y))
    }
}

impl Fract for Vec3 {
    fn fract(self) -> Self {
        Vec3::new(
            Fract::fract(self.x),
            Fract::fract(self.y),
            Fract::fract(self.z),
        )
    }
}

impl Fract for Vec4 {
    fn fract(self) -> Self {
        Vec4::new(
            Fract::fract(self.x),
            Fract::fract(self.y),
            Fract::fract(self.z),
            Fract::fract(self.w),
        )
    }
}

