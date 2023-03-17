//! WGSL `sin()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `sqrt()` function.
///
/// Returns the square root of self.
pub trait Sin {
    fn sin(self) -> Self;
}

impl Sin for f32 {
    fn sin(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::sin(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::sin(self)
        }
    }
}

impl Sin for Vec2 {
    fn sin(self) -> Self {
        Vec2::new(Sin::sin(self.x), Sin::sin(self.y))
    }
}

impl Sin for Vec3 {
    fn sin(self) -> Self {
        Vec3::new(Sin::sin(self.x), Sin::sin(self.y), Sin::sin(self.z))
    }
}

impl Sin for Vec4 {
    fn sin(self) -> Self {
        Vec4::new(
            Sin::sin(self.x),
            Sin::sin(self.y),
            Sin::sin(self.z),
            Sin::sin(self.w),
        )
    }
}
