//! WGSL `smoothstep()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `log2()` function.
///
/// Returns 2 raised to the power of x.
pub trait Log2 {
    fn log2(self) -> Self;
}

impl Log2 for f32 {
    fn log2(self) -> Self {
        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::log2(self)
        }

        #[cfg(feature = "glam")]
        {
            self.log2()
        }
    }
}

impl Log2 for Vec2 {
    fn log2(self) -> Self {
        Vec2::new(Log2::log2(self.x), Log2::log2(self.y))
    }
}

impl Log2 for Vec3 {
    fn log2(self) -> Self {
        Vec3::new(Log2::log2(self.x), Log2::log2(self.y), Log2::log2(self.z))
    }
}

impl Log2 for Vec4 {
    fn log2(self) -> Self {
        Vec4::new(
            Log2::log2(self.x),
            Log2::log2(self.y),
            Log2::log2(self.z),
            Log2::log2(self.w),
        )
    }
}
