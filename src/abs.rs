//! Shader `abs()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Returns a positive-signed version of self.
///
/// Equivalent of the `abs()` function.
pub trait Abs {
    fn abs(self) -> Self;
}

impl Abs for i32 {
    fn abs(self) -> Self {
        i32::abs(self)
    }
}

impl Abs for f32 {
    fn abs(self) -> Self {
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

impl Abs for f64 {
    fn abs(self) -> Self {
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

impl Abs for Vec2 {
    fn abs(self) -> Self {
        Vec2::abs(self)
    }
}

impl Abs for Vec3 {
    fn abs(self) -> Self {
        Vec3::abs(self)
    }
}

impl Abs for Vec4 {
    fn abs(self) -> Self {
        Vec4::abs(self)
    }
}

impl Abs for DVec2 {
    fn abs(self) -> Self {
        DVec2::abs(self)
    }
}

impl Abs for DVec3 {
    fn abs(self) -> Self {
        DVec3::abs(self)
    }
}

impl Abs for DVec4 {
    fn abs(self) -> Self {
        DVec4::abs(self)
    }
}
