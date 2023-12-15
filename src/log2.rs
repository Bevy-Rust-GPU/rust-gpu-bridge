//! Shader `log2()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the base-2 logarithm of self.
///
/// Equivalent of the `log2()` function.
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

impl Log2 for f64 {
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

impl Log2 for DVec2 {
    fn log2(self) -> Self {
        DVec2::new(Log2::log2(self.x), Log2::log2(self.y))
    }
}

impl Log2 for DVec3 {
    fn log2(self) -> Self {
        DVec3::new(Log2::log2(self.x), Log2::log2(self.y), Log2::log2(self.z))
    }
}

impl Log2 for DVec4 {
    fn log2(self) -> Self {
        DVec4::new(
            Log2::log2(self.x),
            Log2::log2(self.y),
            Log2::log2(self.z),
            Log2::log2(self.w),
        )
    }
}
