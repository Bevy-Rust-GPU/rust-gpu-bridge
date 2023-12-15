//! Shader `cos()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the cosine of self.
///
/// Equivalent of the `cos()` function.
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

impl Cos for f64 {
    fn cos(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::cos(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::cos(self)
        }
    }
}

impl Cos for Vec2 {
    fn cos(self) -> Self {
        Vec2::new(Cos::cos(self.x), Cos::cos(self.y))
    }
}

impl Cos for Vec3 {
    fn cos(self) -> Self {
        Vec3::new(Cos::cos(self.x), Cos::cos(self.y), Cos::cos(self.z))
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

impl Cos for DVec2 {
    fn cos(self) -> Self {
        DVec2::new(Cos::cos(self.x), Cos::cos(self.y))
    }
}

impl Cos for DVec3 {
    fn cos(self) -> Self {
        DVec3::new(Cos::cos(self.x), Cos::cos(self.y), Cos::cos(self.z))
    }
}

impl Cos for DVec4 {
    fn cos(self) -> Self {
        DVec4::new(
            Cos::cos(self.x),
            Cos::cos(self.y),
            Cos::cos(self.z),
            Cos::cos(self.w),
        )
    }
}
