//! Shader `sqrt()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the square root of self.
///
/// Equivalent of the `sqrt()` function.
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

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::sqrt(self)
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

impl Sqrt for DVec2 {
    fn sqrt(self) -> Self {
        DVec2::new(Sqrt::sqrt(self.x), Sqrt::sqrt(self.y))
    }
}

impl Sqrt for DVec3 {
    fn sqrt(self) -> Self {
        DVec3::new(Sqrt::sqrt(self.x), Sqrt::sqrt(self.y), Sqrt::sqrt(self.z))
    }
}

impl Sqrt for DVec4 {
    fn sqrt(self) -> Self {
        DVec4::new(
            Sqrt::sqrt(self.x),
            Sqrt::sqrt(self.y),
            Sqrt::sqrt(self.z),
            Sqrt::sqrt(self.w),
        )
    }
}
