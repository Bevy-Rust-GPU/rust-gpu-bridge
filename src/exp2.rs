//! Shader `exp2()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns 2 raised to the power of self.
///
/// Equivalent of the `exp2()` function.
pub trait Exp2 {
    fn exp2(self) -> Self;
}

impl Exp2 for f32 {
    fn exp2(self) -> Self {
        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::exp2(self)
        }

        #[cfg(feature = "glam")]
        {
            2.0_f32.powf(self)
        }
    }
}

impl Exp2 for f64 {
    fn exp2(self) -> Self {
        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::exp2(self)
        }

        #[cfg(feature = "glam")]
        {
            2.0_f64.powf(self)
        }
    }
}

impl Exp2 for Vec2 {
    fn exp2(self) -> Self {
        Vec2::new(Exp2::exp2(self.x), Exp2::exp2(self.y))
    }
}

impl Exp2 for Vec3 {
    fn exp2(self) -> Self {
        Vec3::new(Exp2::exp2(self.x), Exp2::exp2(self.y), Exp2::exp2(self.z))
    }
}

impl Exp2 for Vec4 {
    fn exp2(self) -> Self {
        Vec4::new(
            Exp2::exp2(self.x),
            Exp2::exp2(self.y),
            Exp2::exp2(self.z),
            Exp2::exp2(self.w),
        )
    }
}

impl Exp2 for DVec2 {
    fn exp2(self) -> Self {
        DVec2::new(Exp2::exp2(self.x), Exp2::exp2(self.y))
    }
}

impl Exp2 for DVec3 {
    fn exp2(self) -> Self {
        DVec3::new(Exp2::exp2(self.x), Exp2::exp2(self.y), Exp2::exp2(self.z))
    }
}

impl Exp2 for DVec4 {
    fn exp2(self) -> Self {
        DVec4::new(
            Exp2::exp2(self.x),
            Exp2::exp2(self.y),
            Exp2::exp2(self.z),
            Exp2::exp2(self.w),
        )
    }
}
