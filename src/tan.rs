//! WGSL `tan()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `tan()` function.
///
/// Returns the square root of self.
pub trait Tan {
    fn tan(self) -> Self;
}

impl Tan for f32 {
    fn tan(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::tan(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::tan(self)
        }
    }
}

impl Tan for Vec2 {
    fn tan(self) -> Self {
        Vec2::new(Tan::tan(self.x), Tan::tan(self.y))
    }
}

impl Tan for Vec3 {
    fn tan(self) -> Self {
        Vec3::new(Tan::tan(self.x), Tan::tan(self.y), Tan::tan(self.z))
    }
}

impl Tan for Vec4 {
    fn tan(self) -> Self {
        Vec4::new(
            Tan::tan(self.x),
            Tan::tan(self.y),
            Tan::tan(self.z),
            Tan::tan(self.w),
        )
    }
}
