//! WGSL `round()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `round()` function.
///
/// Returns self rounded to the nearest integer.
pub trait Round {
    fn round(self) -> Self;
}

impl Round for f32 {
    fn round(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::round(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::round(self)
        }
    }
}

impl Round for Vec2 {
    fn round(self) -> Self {
        Vec2::new(Round::round(self.x), Round::round(self.y))
    }
}

impl Round for Vec3 {
    fn round(self) -> Self {
        Vec3::new(Round::round(self.x), Round::round(self.y), Round::round(self.z))
    }
}

impl Round for Vec4 {
    fn round(self) -> Self {
        Vec4::new(
            Round::round(self.x),
            Round::round(self.y),
            Round::round(self.z),
            Round::round(self.w),
        )
    }
}

