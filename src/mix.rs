//! Shader `mix()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the linear interpolation between self and to at t.
///
/// Equivalent of the `mix()` function.
pub trait Mix {
    type T;
    fn mix(self, to: Self, t: Self::T) -> Self;
}

impl Mix for f32 {
    type T = Self;
    fn mix(self, to: Self, t: Self::T) -> Self {
        (1.0 - t) * self + t * to
    }
}

impl Mix for f64 {
    type T = Self;
    fn mix(self, to: Self, t: Self::T) -> Self {
        (1.0 - t) * self + t * to
    }
}

impl Mix for Vec2 {
    type T = f32;

    fn mix(self, to: Self, t: Self::T) -> Self {
        Vec2::new(self.x.mix(to.x, t), self.y.mix(to.y, t))
    }
}

impl Mix for Vec3 {
    type T = f32;

    fn mix(self, to: Self, t: Self::T) -> Self {
        Vec3::new(
            self.x.mix(to.x, t),
            self.y.mix(to.y, t),
            self.z.mix(to.z, t),
        )
    }
}

impl Mix for Vec4 {
    type T = f32;

    fn mix(self, to: Self, t: Self::T) -> Self {
        Vec4::new(
            self.x.mix(to.x, t),
            self.y.mix(to.y, t),
            self.z.mix(to.z, t),
            self.w.mix(to.w, t),
        )
    }
}

impl Mix for DVec2 {
    type T = f64;

    fn mix(self, to: Self, t: Self::T) -> Self {
        DVec2::new(self.x.mix(to.x, t), self.y.mix(to.y, t))
    }
}

impl Mix for DVec3 {
    type T = f64;

    fn mix(self, to: Self, t: Self::T) -> Self {
        DVec3::new(
            self.x.mix(to.x, t),
            self.y.mix(to.y, t),
            self.z.mix(to.z, t),
        )
    }
}

impl Mix for DVec4 {
    type T = f64;

    fn mix(self, to: Self, t: Self::T) -> Self {
        DVec4::new(
            self.x.mix(to.x, t),
            self.y.mix(to.y, t),
            self.z.mix(to.z, t),
            self.w.mix(to.w, t),
        )
    }
}
