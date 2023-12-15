//! Shader `atan2()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the two-argument arctangent of self and some other value
///
/// Equivalent of the `atan2()` function.
pub trait Atan2 {
    fn atan2(self, p: Self) -> Self;
}

impl Atan2 for f32 {
    fn atan2(self, p: Self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::atan2(self, p)
        }

        #[cfg(feature = "spirv-std")]
        {
            <f32 as spirv_std::num_traits::Float>::atan2(self, p)
        }
    }
}

impl Atan2 for f64 {
    fn atan2(self, p: Self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::atan2(self, p)
        }

        #[cfg(feature = "spirv-std")]
        {
            <f64 as spirv_std::num_traits::Float>::atan2(self, p)
        }
    }
}

impl Atan2 for Vec2 {
    fn atan2(self, p: Self) -> Self {
        Vec2::new(Atan2::atan2(self.x, p.x), Atan2::atan2(self.y, p.y))
    }
}

impl Atan2 for Vec3 {
    fn atan2(self, p: Self) -> Self {
        Vec3::new(
            Atan2::atan2(self.x, p.x),
            Atan2::atan2(self.y, p.y),
            Atan2::atan2(self.z, p.z),
        )
    }
}

impl Atan2 for Vec4 {
    fn atan2(self, p: Self) -> Self {
        Vec4::new(
            Atan2::atan2(self.x, p.x),
            Atan2::atan2(self.y, p.y),
            Atan2::atan2(self.z, p.z),
            Atan2::atan2(self.w, p.w),
        )
    }
}

impl Atan2 for DVec2 {
    fn atan2(self, p: Self) -> Self {
        DVec2::new(Atan2::atan2(self.x, p.x), Atan2::atan2(self.y, p.y))
    }
}

impl Atan2 for DVec3 {
    fn atan2(self, p: Self) -> Self {
        DVec3::new(
            Atan2::atan2(self.x, p.x),
            Atan2::atan2(self.y, p.y),
            Atan2::atan2(self.z, p.z),
        )
    }
}

impl Atan2 for DVec4 {
    fn atan2(self, p: Self) -> Self {
        DVec4::new(
            Atan2::atan2(self.x, p.x),
            Atan2::atan2(self.y, p.y),
            Atan2::atan2(self.z, p.z),
            Atan2::atan2(self.w, p.w),
        )
    }
}
