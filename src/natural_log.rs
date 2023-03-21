//! Shader `log()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the natural logarithm (base ~2.7182..) of the provided value.
///
/// Equivalent of the `log()` function.
pub trait NaturalLog {
    const BASE: f32 = 2.718281828459;

    fn natural_log(&self) -> Self;
}

impl NaturalLog for f32 {
    fn natural_log(&self) -> Self {
        self.log(Self::BASE)
    }
}

impl NaturalLog for Vec2 {
    fn natural_log(&self) -> Self {
        Vec2::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
        )
    }
}

impl NaturalLog for Vec3 {
    fn natural_log(&self) -> Self {
        Vec3::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
            NaturalLog::natural_log(self.z),
        )
    }
}

impl NaturalLog for Vec4 {
    fn natural_log(&self) -> Self {
        Vec4::new(
            NaturalLog::natural_log(self.x),
            NaturalLog::natural_log(self.y),
            NaturalLog::natural_log(self.z),
            NaturalLog::natural_log(self.w),
        )
    }
}
