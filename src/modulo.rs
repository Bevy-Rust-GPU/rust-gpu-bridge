//! Shader `mod()`

use crate::glam::{Vec2, Vec3, Vec4};

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
            self.rem_euclid(&modulus)
        }

        #[cfg(feature = "glam")]
        {
            self.rem_euclid(modulus)
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
