//! WGSL `atan()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Computes the arcsine of self.
///
/// Equivalent of the `atan()` function.
pub trait Atan {
    fn atan(self) -> Self;
}

impl Atan for f32 {
    fn atan(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::atan(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::atan(self)
        }
    }
}

impl Atan for f64 {
    fn atan(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::atan(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::atan(self)
        }
    }
}

impl Atan for Vec2 {
    fn atan(self) -> Self {
        Vec2::new(self.x.atan(), self.y.atan())
    }
}

impl Atan for Vec3 {
    fn atan(self) -> Self {
        Vec3::new(self.x.atan(), self.y.atan(), self.z.atan())
    }
}

impl Atan for Vec4 {
    fn atan(self) -> Self {
        Vec4::new(self.x.atan(), self.y.atan(), self.z.atan(), self.w.atan())
    }
}

impl Atan for DVec2 {
    fn atan(self) -> Self {
        DVec2::new(self.x.atan(), self.y.atan())
    }
}

impl Atan for DVec3 {
    fn atan(self) -> Self {
        DVec3::new(self.x.atan(), self.y.atan(), self.z.atan())
    }
}

impl Atan for DVec4 {
    fn atan(self) -> Self {
        DVec4::new(self.x.atan(), self.y.atan(), self.z.atan(), self.w.atan())
    }
}

