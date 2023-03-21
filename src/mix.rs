//! Shader `mix()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Computes the linear interpolation between self and to at t.
///
/// Equivalent of the `mix()` function.
pub trait Mix {
    fn mix(self, to: Self, t: Self) -> Self;
}

impl Mix for f32 {
    fn mix(self, to: Self, t: Self) -> Self {
        (1.0 - t) * self + t * to
    }
}

impl Mix for Vec2 {
    fn mix(self, to: Self, t: Self) -> Self {
        Vec2::new(self.x.mix(to.x, t.x), self.y.mix(to.y, t.y))
    }
}

impl Mix for Vec3 {
    fn mix(self, to: Self, t: Self) -> Self {
        Vec3::new(
            self.x.mix(to.x, t.x),
            self.y.mix(to.y, t.y),
            self.z.mix(to.z, t.z),
        )
    }
}

impl Mix for Vec4 {
    fn mix(self, to: Self, t: Self) -> Self {
        Vec4::new(
            self.x.mix(to.x, t.x),
            self.y.mix(to.y, t.y),
            self.z.mix(to.z, t.z),
            self.w.mix(to.w, t.w),
        )
    }
}
