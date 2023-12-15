//! Shader `sign()`

use crate::glam::{Vec2, Vec3, Vec4};

use glam::{DVec2, DVec3, DVec4};
#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Returns -1 for negative numbers, and 1.0 for positive numbers.
///
/// Equivalent of the `sign()` function.
pub trait Sign {
    fn sign(self) -> Self;
}

impl Sign for i8 {
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Sign for i16 {
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Sign for i32 {
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Sign for i64 {
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Sign for f32 {
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Sign for f64 {
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Sign for Vec2 {
    fn sign(self) -> Self {
        Vec2::new(self.x.signum(), self.y.signum())
    }
}

impl Sign for Vec3 {
    fn sign(self) -> Self {
        Vec3::new(self.x.signum(), self.y.signum(), self.z.signum())
    }
}

impl Sign for Vec4 {
    fn sign(self) -> Self {
        Vec4::new(
            self.x.signum(),
            self.y.signum(),
            self.z.signum(),
            self.w.signum(),
        )
    }
}

impl Sign for DVec2 {
    fn sign(self) -> Self {
        DVec2::new(self.x.signum(), self.y.signum())
    }
}

impl Sign for DVec3 {
    fn sign(self) -> Self {
        DVec3::new(self.x.signum(), self.y.signum(), self.z.signum())
    }
}

impl Sign for DVec4 {
    fn sign(self) -> Self {
        DVec4::new(
            self.x.signum(),
            self.y.signum(),
            self.z.signum(),
            self.w.signum(),
        )
    }
}
