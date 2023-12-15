//! Smooth `smoothstep()`

use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

/// Returns the smooth Hermite interpolation between 0.0 and 1.0.
///
/// Equivalent of the `smoothstep()` function.
pub trait SmoothStep {
    type T;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self;
}

impl SmoothStep for f32 {
    type T = Self;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        // Scale/bias into [0..1] range
        let x = ((self - edge_in) / (edge_out - edge_in)).clamp(0.0, 1.0);

        return x * x * (3.0 - 2.0 * x);
    }
}

impl SmoothStep for f64 {
    type T = Self;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        // Scale/bias into [0..1] range
        let x = ((self - edge_in) / (edge_out - edge_in)).clamp(0.0, 1.0);

        return x * x * (3.0 - 2.0 * x);
    }
}

impl SmoothStep for Vec2 {
    type T = f32;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        Vec2::new(
            self.x.smooth_step(edge_in, edge_out),
            self.y.smooth_step(edge_in, edge_out),
        )
    }
}

impl SmoothStep for Vec3 {
    type T = f32;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        Vec3::new(
            self.x.smooth_step(edge_in, edge_out),
            self.y.smooth_step(edge_in, edge_out),
            self.z.smooth_step(edge_in, edge_out),
        )
    }
}

impl SmoothStep for Vec4 {
    type T = f32;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        Vec4::new(
            self.x.smooth_step(edge_in, edge_out),
            self.y.smooth_step(edge_in, edge_out),
            self.z.smooth_step(edge_in, edge_out),
            self.w.smooth_step(edge_in, edge_out),
        )
    }
}

impl SmoothStep for DVec2 {
    type T = f64;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        DVec2::new(
            self.x.smooth_step(edge_in, edge_out),
            self.y.smooth_step(edge_in, edge_out),
        )
    }
}

impl SmoothStep for DVec3 {
    type T = f64;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        DVec3::new(
            self.x.smooth_step(edge_in, edge_out),
            self.y.smooth_step(edge_in, edge_out),
            self.z.smooth_step(edge_in, edge_out),
        )
    }
}

impl SmoothStep for DVec4 {
    type T = f64;

    fn smooth_step(self, edge_in: Self::T, edge_out: Self::T) -> Self {
        DVec4::new(
            self.x.smooth_step(edge_in, edge_out),
            self.y.smooth_step(edge_in, edge_out),
            self.z.smooth_step(edge_in, edge_out),
            self.w.smooth_step(edge_in, edge_out),
        )
    }
}
