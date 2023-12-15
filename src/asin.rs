//! WGSL `asin()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Computes the arcsine of self.
///
/// Equivalent of the `asin()` function.
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

impl Asin for f64 {
    fn asin(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::asin(self)
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


impl Asin for DVec2 {
    fn asin(self) -> Self {
        DVec2::new(self.x.asin(), self.y.asin())
    }
}

impl Asin for DVec3 {
    fn asin(self) -> Self {
        DVec3::new(self.x.asin(), self.y.asin(), self.z.asin())
    }
}

impl Asin for DVec4 {
    fn asin(self) -> Self {
        DVec4::new(self.x.asin(), self.y.asin(), self.z.asin(), self.w.asin())
    }
}

