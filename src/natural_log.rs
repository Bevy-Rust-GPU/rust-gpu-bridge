//! Shader `log()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the natural logarithm (base ~2.7182..) of the provided value.
///
/// Equivalent of the `log()` function.
pub trait NaturalLog {
    type Base;
    const BASE: Self::Base;

    fn natural_log(self) -> Self;
}

impl NaturalLog for f32 {
    type Base = Self;
    const BASE: Self::Base = 2.718281828459;

    fn natural_log(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::log(self, Self::BASE)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::log(self, Self::BASE)
        }
    }
}

impl NaturalLog for f64 {
    type Base = Self;
    const BASE: Self::Base = 2.718281828459;

    fn natural_log(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::log(self, Self::BASE)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::log(self, Self::BASE)
        }
    }
}

impl NaturalLog for Vec2 {
    type Base = f32;
    const BASE: Self::Base = f32::BASE;

    fn natural_log(self) -> Self {
        Vec2::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
        )
    }
}

impl NaturalLog for Vec3 {
    type Base = f32;
    const BASE: Self::Base = f32::BASE;

    fn natural_log(self) -> Self {
        Vec3::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
            NaturalLog::natural_log(self.z),
        )
    }
}

impl NaturalLog for Vec4 {
    type Base = f32;
    const BASE: Self::Base = f32::BASE;

    fn natural_log(self) -> Self {
        Vec4::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
            NaturalLog::natural_log(self.z),
            NaturalLog::natural_log(self.w),
        )
    }
}

impl NaturalLog for DVec2 {
    type Base = f64;
    const BASE: Self::Base = f64::BASE;

    fn natural_log(self) -> Self {
        DVec2::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
        )
    }
}

impl NaturalLog for DVec3 {
    type Base = f64;
    const BASE: Self::Base = f64::BASE;

    fn natural_log(self) -> Self {
        DVec3::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
            NaturalLog::natural_log(self.z),
        )
    }
}

impl NaturalLog for DVec4 {
    type Base = f64;
    const BASE: Self::Base = f64::BASE;

    fn natural_log(self) -> Self {
        DVec4::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
            NaturalLog::natural_log(self.z),
            NaturalLog::natural_log(self.w),
        )
    }
}
