//! WGSL `sqrt()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `sqrt()` function.
///
/// Returns the square root of self.
pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::sqrt(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::sqrt(self)
        }
    }
}

impl Sqrt for Vec2 {
    fn sqrt(self) -> Self {
        Vec2::new(Sqrt::sqrt(self.x), Sqrt::sqrt(self.y))
    }
}

impl Sqrt for Vec3 {
    fn sqrt(self) -> Self {
        Vec3::new(Sqrt::sqrt(self.x), Sqrt::sqrt(self.y), Sqrt::sqrt(self.z))
    }
}

impl Sqrt for Vec4 {
    fn sqrt(self) -> Self {
        Vec4::new(
            Sqrt::sqrt(self.x),
            Sqrt::sqrt(self.y),
            Sqrt::sqrt(self.z),
            Sqrt::sqrt(self.w),
        )
    }
}
