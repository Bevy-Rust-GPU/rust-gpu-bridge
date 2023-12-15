//! Shader `normalize()`

use crate::{
    glam::{Vec2, Vec3, Vec4},
    sign::Sign,
};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns self scaled such that length is 1.
///
/// Equivalent of the `normalize()` function.
pub trait Normalize {
    fn normalize(self) -> Self;
}

impl Normalize for f32 {
    fn normalize(self) -> Self {
        self.sign()
    }
}

impl Normalize for f64 {
    fn normalize(self) -> Self {
        self.sign()
    }
}

impl Normalize for Vec2 {
    fn normalize(self) -> Self {
        Vec2::normalize_or_zero(self)
    }
}

impl Normalize for Vec3 {
    fn normalize(self) -> Self {
        Vec3::normalize_or_zero(self)
    }
}

impl Normalize for Vec4 {
    fn normalize(self) -> Self {
        Vec4::normalize_or_zero(self)
    }
}

impl Normalize for DVec2 {
    fn normalize(self) -> Self {
        DVec2::normalize_or_zero(self)
    }
}

impl Normalize for DVec3 {
    fn normalize(self) -> Self {
        DVec3::normalize_or_zero(self)
    }
}

impl Normalize for DVec4 {
    fn normalize(self) -> Self {
        DVec4::normalize_or_zero(self)
    }
}
