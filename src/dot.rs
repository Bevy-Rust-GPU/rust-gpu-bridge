//! Shader `dot()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns a scalar from -1.0..1.0 denoting the similarity of two quantities.
///
/// Equivalent of the `dot()` function.
pub trait Dot {
    type Output;
    fn dot(self, rhs: Self) -> Self::Output;
}

impl Dot for f32 {
    type Output = f32;

    fn dot(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl Dot for f64 {
    type Output = f64;

    fn dot(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl Dot for Vec2 {
    type Output = f32;

    fn dot(self, rhs: Self) -> Self::Output {
        Vec2::dot(self, rhs)
    }
}

impl Dot for Vec3 {
    type Output = f32;

    fn dot(self, rhs: Self) -> Self::Output {
        Vec3::dot(self, rhs)
    }
}

impl Dot for Vec4 {
    type Output = f32;

    fn dot(self, rhs: Self) -> Self::Output {
        Vec4::dot(self, rhs)
    }
}

impl Dot for DVec2 {
    type Output = f64;

    fn dot(self, rhs: Self) -> Self::Output {
        DVec2::dot(self, rhs)
    }
}

impl Dot for DVec3 {
    type Output = f64;

    fn dot(self, rhs: Self) -> Self::Output {
        DVec3::dot(self, rhs)
    }
}

impl Dot for DVec4 {
    type Output = f64;

    fn dot(self, rhs: Self) -> Self::Output {
        DVec4::dot(self, rhs)
    }
}
