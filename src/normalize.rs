//! WGSL `normalize()`

use crate::{
    glam::{Vec2, Vec3, Vec4},
    sign::Sign,
};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `normalize()` function.
///
/// Returns the provided vector quantity scaled such that its length is 1.
pub trait Normalize {
    fn normalize(self) -> Self;
}

impl Normalize for f32 {
    fn normalize(self) -> Self {
        self.sign()
    }
}

impl Normalize for Vec2 {
    fn normalize(self) -> Self {
        Vec2::normalize(self)
    }
}

impl Normalize for Vec3 {
    fn normalize(self) -> Self {
        Vec3::normalize(self)
    }
}

impl Normalize for Vec4 {
    fn normalize(self) -> Self {
        Vec4::normalize(self)
    }
}

