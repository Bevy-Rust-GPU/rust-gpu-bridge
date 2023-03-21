//! Shader `dot()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns a scalar from -1.0..1.0 denoting the similarity of two quantities.
///
/// Equivalent of the `dot()` function.
pub trait Dot {
    fn dot(self, rhs: Self) -> f32;
}

impl Dot for f32 {
    fn dot(self, rhs: Self) -> f32 {
        self * rhs
    }
}

impl Dot for Vec2 {
    fn dot(self, rhs: Self) -> f32 {
        Vec2::dot(self, rhs)
    }
}

impl Dot for Vec3 {
    fn dot(self, rhs: Self) -> f32 {
        Vec3::dot(self, rhs)
    }
}

impl Dot for Vec4 {
    fn dot(self, rhs: Self) -> f32 {
        Vec4::dot(self, rhs)
    }
}

