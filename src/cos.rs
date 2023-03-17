//! WGSL `cos()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `sqrt()` function.
///
/// Returns the square root of self.
pub trait Cos {
    fn cos(self) -> Self;
}

impl Cos for f32 {
    fn cos(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::cos(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::cos(self)
        }
    }
}

impl Cos for Vec2 {
    fn cos(self) -> Self {
        Vec2::new(
            Cos::cos(self.x),
            Cos::cos(self.y),
        )
    }
}

impl Cos for Vec3 {
    fn cos(self) -> Self {
        Vec3::new(
            Cos::cos(self.x),
            Cos::cos(self.y),
            Cos::cos(self.z),
        )
    }
}

impl Cos for Vec4 {
    fn cos(self) -> Self {
        Vec4::new(
            Cos::cos(self.x),
            Cos::cos(self.y),
            Cos::cos(self.z),
            Cos::cos(self.w),
        )
    }
}
