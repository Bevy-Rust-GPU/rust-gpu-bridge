//! WGSL `asin()`

use crate::glam::{Vec2, Vec3, Vec4};

/// Equivalent of the WGSL `saturate()` function.
///
/// Clamps a value to the 0.0..=1.0 range
pub trait Asin {
    fn asin(self) -> Self;
}

impl Asin for f32 {
    fn asin(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::asin(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::asin(self)
        }
    }
}

impl Asin for Vec2 {
    fn asin(self) -> Self {
        Vec2::new(self.x.asin(), self.y.asin())
    }
}

impl Asin for Vec3 {
    fn asin(self) -> Self {
        Vec3::new(self.x.asin(), self.y.asin(), self.z.asin())
    }
}

impl Asin for Vec4 {
    fn asin(self) -> Self {
        Vec4::new(self.x.asin(), self.y.asin(), self.z.asin(), self.w.asin())
    }
}

