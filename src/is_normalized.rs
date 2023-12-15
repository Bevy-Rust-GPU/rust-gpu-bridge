//! Shader `is_normalized()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns true if self is normalized, false otherwise
///
/// Equivalent of the `is_normalized()` function.
pub trait IsNormalized {
    fn is_normalized(self) -> bool;
}

impl IsNormalized for f32 {
    fn is_normalized(self) -> bool {
        #[cfg(feature = "glam")]
        {
            f32::abs(self) > 1e-4
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::abs(self) > 1e-4
        }
    }
}

impl IsNormalized for f64 {
    fn is_normalized(self) -> bool {
        #[cfg(feature = "glam")]
        {
            f64::abs(self) > 1e-4
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::abs(self) > 1e-4
        }
    }
}

impl IsNormalized for Vec2 {
    fn is_normalized(self) -> bool {
        Vec2::is_normalized(self)
    }
}

impl IsNormalized for Vec3 {
    fn is_normalized(self) -> bool {
        Vec3::is_normalized(self)
    }
}

impl IsNormalized for Vec4 {
    fn is_normalized(self) -> bool {
        Vec4::is_normalized(self)
    }
}

impl IsNormalized for DVec2 {
    fn is_normalized(self) -> bool {
        DVec2::is_normalized(self)
    }
}

impl IsNormalized for DVec3 {
    fn is_normalized(self) -> bool {
        DVec3::is_normalized(self)
    }
}

impl IsNormalized for DVec4 {
    fn is_normalized(self) -> bool {
        DVec4::is_normalized(self)
    }
}
