//! Shader `acos()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Computes the arcsine of self.
///
/// Equivalent of the `acos()` function.
pub trait Acos {
    fn acos(self) -> Self;
}

impl Acos for f32 {
    fn acos(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::acos(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::acos(self)
        }
    }
}

impl Acos for f64 {
    fn acos(self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::acos(self)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::acos(self)
        }
    }
}

impl Acos for Vec2 {
    fn acos(self) -> Self {
        Vec2::new(self.x.acos(), self.y.acos())
    }
}

impl Acos for Vec3 {
    fn acos(self) -> Self {
        Vec3::new(self.x.acos(), self.y.acos(), self.z.acos())
    }
}

impl Acos for Vec4 {
    fn acos(self) -> Self {
        Vec4::new(self.x.acos(), self.y.acos(), self.z.acos(), self.w.acos())
    }
}

impl Acos for DVec2 {
    fn acos(self) -> Self {
        DVec2::new(self.x.acos(), self.y.acos())
    }
}

impl Acos for DVec3 {
    fn acos(self) -> Self {
        DVec3::new(self.x.acos(), self.y.acos(), self.z.acos())
    }
}

impl Acos for DVec4 {
    fn acos(self) -> Self {
        DVec4::new(self.x.acos(), self.y.acos(), self.z.acos(), self.w.acos())
    }
}
