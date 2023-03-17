//! WGSL `sign()`

use crate::glam::{Vec2, Vec3, Vec4};

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `sign()` function.
///
/// Returns self raised to the p-th power.
pub trait Sign {
    fn sign(self) -> Self;
}

impl Sign for f32 {
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
