//! Shader `saturate()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Clamps self to the 0.0..=1.0 range.
///
/// Equivalent of the `saturate()` function.
pub trait Saturate {
    fn saturate(self) -> Self;
}

impl Saturate for f32 {
    fn saturate(self) -> Self {
        self.clamp(0.0, 1.0)
    }
}

impl Saturate for f64 {
    fn saturate(self) -> Self {
        self.clamp(0.0, 1.0)
    }
}

impl Saturate for Vec2 {
    fn saturate(self) -> Self {
        Vec2::new(self.x.saturate(), self.y.saturate())
    }
}

impl Saturate for Vec3 {
    fn saturate(self) -> Self {
        Vec3::new(self.x.saturate(), self.y.saturate(), self.z.saturate())
    }
}

impl Saturate for Vec4 {
    fn saturate(self) -> Self {
        Vec4::new(
            self.x.saturate(),
            self.y.saturate(),
            self.z.saturate(),
            self.w.saturate(),
        )
    }
}

impl Saturate for DVec2 {
    fn saturate(self) -> Self {
        DVec2::new(self.x.saturate(), self.y.saturate())
    }
}

impl Saturate for DVec3 {
    fn saturate(self) -> Self {
        DVec3::new(self.x.saturate(), self.y.saturate(), self.z.saturate())
    }
}

impl Saturate for DVec4 {
    fn saturate(self) -> Self {
        DVec4::new(
            self.x.saturate(),
            self.y.saturate(),
            self.z.saturate(),
            self.w.saturate(),
        )
    }
}
