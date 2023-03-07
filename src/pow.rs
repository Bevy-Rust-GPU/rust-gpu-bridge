//! WGSL `smoothstep()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `smoothstep()` function.
///
/// Returns the smooth Hermite interpolation between 0.0 and 1.0.
pub trait Pow {
    fn pow(self, p: Self) -> Self;
}

impl Pow for f32 {
    fn pow(self, p: Self) -> Self {
        self.powf(p)
    }
}

impl Pow for Vec2 {
    fn pow(self, p: Self) -> Self {
        Vec2::new(self.x.pow(p.x), self.y.pow(p.y))
    }
}

impl Pow for Vec3 {
    fn pow(self, p: Self) -> Self {
        Vec3::new(
            self.x.pow(p.x),
            self.y.pow(p.y),
            self.z.pow(p.z),
        )
    }
}

impl Pow for Vec4 {
    fn pow(self, p: Self) -> Self {
        Vec4::new(
            self.x.pow(p.x),
            self.y.pow(p.y),
            self.z.pow(p.z),
            self.w.pow(p.w),
        )
    }
}
