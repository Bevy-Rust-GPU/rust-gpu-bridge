//! Shader `pow()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns self raised to the nth power.
///
/// Equivalent of the `pow()` function.
pub trait Pow {
    fn pow(self, p: Self) -> Self;
}

impl Pow for f32 {
    fn pow(self, p: Self) -> Self {
        self.powf(p)
    }
}

impl Pow for f64 {
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
        Vec3::new(self.x.pow(p.x), self.y.pow(p.y), self.z.pow(p.z))
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

impl Pow for DVec2 {
    fn pow(self, p: Self) -> Self {
        DVec2::new(self.x.pow(p.x), self.y.pow(p.y))
    }
}

impl Pow for DVec3 {
    fn pow(self, p: Self) -> Self {
        DVec3::new(self.x.pow(p.x), self.y.pow(p.y), self.z.pow(p.z))
    }
}

impl Pow for DVec4 {
    fn pow(self, p: Self) -> Self {
        DVec4::new(
            self.x.pow(p.x),
            self.y.pow(p.y),
            self.z.pow(p.z),
            self.w.pow(p.w),
        )
    }
}
