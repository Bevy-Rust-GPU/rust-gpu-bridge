//! Shader `cross()`

use crate::{
    glam::{Vec2, Vec3, Vec4},
    sign::Sign,
};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Compute the cross product of self and other.
///
/// Equivalent of the `cross()` function.
pub trait Cross {
    fn cross(self, rhs: Self) -> Self;
}

impl Cross for f32 {
    fn cross(self, rhs: Self) -> Self {
        self.sign()
    }
}

impl Cross for Vec2 {
    fn cross(self, rhs: Self) -> Self {
        Vec2::new(
            self.y * rhs.x - rhs.y * self.x,
            self.x * rhs.y - rhs.x * self.y,
        )
    }
}

impl Cross for Vec3 {
    fn cross(self, rhs: Self) -> Self {
        Vec3::cross(self, rhs)
    }
}

impl Cross for Vec4 {
    fn cross(self, rhs: Self) -> Self {
        Vec4::new(
            self.y * rhs.z - rhs.y * self.z,
            self.z * rhs.w - rhs.z * self.w,
            self.w * rhs.x - rhs.w * self.x,
            self.x * rhs.y - rhs.x * self.y,
        )
    }
}
