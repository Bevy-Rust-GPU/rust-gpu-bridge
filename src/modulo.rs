//! Shader `mod()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
use spirv_std::num_traits::Euclid;

/// Returns self modulo the provided value.
///
/// Equivalent to the `mod()` function
pub trait Mod {
    fn modulo(self, modulus: Self) -> Self;
}

impl Mod for f32 {
    fn modulo(self, modulus: Self) -> Self {
        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Euclid::rem_euclid(&self, &modulus)
        }

        #[cfg(feature = "glam")]
        {
            f32::rem_euclid(self, modulus)
        }
    }
}

impl Mod for f64 {
    fn modulo(self, modulus: Self) -> Self {
        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Euclid::rem_euclid(&self, &modulus)
        }

        #[cfg(feature = "glam")]
        {
            f64::rem_euclid(self, modulus)
        }
    }
}

impl Mod for Vec2 {
    fn modulo(self, modulus: Self) -> Self {
        Vec2::new(self.x.modulo(modulus.x), self.y.modulo(modulus.y))
    }
}

impl Mod for Vec3 {
    fn modulo(self, modulus: Self) -> Self {
        Vec3::new(
            self.x.modulo(modulus.x),
            self.y.modulo(modulus.y),
            self.z.modulo(modulus.z),
        )
    }
}

impl Mod for Vec4 {
    fn modulo(self, modulus: Self) -> Self {
        Vec4::new(
            self.x.modulo(modulus.x),
            self.y.modulo(modulus.y),
            self.z.modulo(modulus.z),
            self.w.modulo(modulus.w),
        )
    }
}

impl Mod for DVec2 {
    fn modulo(self, modulus: Self) -> Self {
        DVec2::new(self.x.modulo(modulus.x), self.y.modulo(modulus.y))
    }
}

impl Mod for DVec3 {
    fn modulo(self, modulus: Self) -> Self {
        DVec3::new(
            self.x.modulo(modulus.x),
            self.y.modulo(modulus.y),
            self.z.modulo(modulus.z),
        )
    }
}

impl Mod for DVec4 {
    fn modulo(self, modulus: Self) -> Self {
        DVec4::new(
            self.x.modulo(modulus.x),
            self.y.modulo(modulus.y),
            self.z.modulo(modulus.z),
            self.w.modulo(modulus.w),
        )
    }
}
