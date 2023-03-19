//! WGSL `step()`

use crate::glam::{Vec2, Vec3, Vec4};

/// Equivalent of the WGSL `step()` function.
///
/// Returns 1.0 if value is >= edge, 0.0 otherwise
pub trait Step {
    fn step(self, edge: f32) -> Self;
}

impl Step for f32 {
    fn step(self, edge: f32) -> Self {
        if edge >= self {
            1.0
        } else {
            0.0
        }
    }
}

impl Step for Vec2 {
    fn step(self, edge: f32) -> Self {
        Vec2::new(self.x.step(edge), self.y.step(edge))
    }
}

impl Step for Vec3 {
    fn step(self, edge: f32) -> Self {
        Vec3::new(self.x.step(edge), self.y.step(edge), self.z.step(edge))
    }
}

impl Step for Vec4 {
    fn step(self, edge: f32) -> Self {
        Vec4::new(
            self.x.step(edge),
            self.y.step(edge),
            self.z.step(edge),
            self.w.step(edge),
        )
    }
}

