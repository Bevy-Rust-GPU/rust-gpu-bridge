//! Shader `step()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Returns 1.0 if value is >= edge, 0.0 otherwise.
///
/// Equivalent of the `step()` function.
pub trait Step {
    type T;

    fn step(self, edge: Self::T) -> Self;
}

impl Step for f32 {
    type T = Self;

    fn step(self, edge: Self::T) -> Self {
        if edge >= self {
            1.0
        } else {
            0.0
        }
    }
}

impl Step for f64 {
    type T = Self;

    fn step(self, edge: Self::T) -> Self {
        if edge >= self {
            1.0
        } else {
            0.0
        }
    }
}

impl Step for Vec2 {
    type T = f32;

    fn step(self, edge: Self::T) -> Self {
        Vec2::new(self.x.step(edge), self.y.step(edge))
    }
}

impl Step for Vec3 {
    type T = f32;

    fn step(self, edge: Self::T) -> Self {
        Vec3::new(self.x.step(edge), self.y.step(edge), self.z.step(edge))
    }
}

impl Step for Vec4 {
    type T = f32;

    fn step(self, edge: Self::T) -> Self {
        Vec4::new(
            self.x.step(edge),
            self.y.step(edge),
            self.z.step(edge),
            self.w.step(edge),
        )
    }
}

impl Step for DVec2 {
    type T = f64;

    fn step(self, edge: Self::T) -> Self {
        DVec2::new(self.x.step(edge), self.y.step(edge))
    }
}

impl Step for DVec3 {
    type T = f64;

    fn step(self, edge: Self::T) -> Self {
        DVec3::new(self.x.step(edge), self.y.step(edge), self.z.step(edge))
    }
}

impl Step for DVec4 {
    type T = f64;

    fn step(self, edge: Self::T) -> Self {
        DVec4::new(
            self.x.step(edge),
            self.y.step(edge),
            self.z.step(edge),
            self.w.step(edge),
        )
    }
}
