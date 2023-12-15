use glam::{DVec2, DVec3, DVec4};

use crate::glam::{Vec2, Vec3, Vec4};

pub trait Splat {
    type T;

    fn splat(value: Self::T) -> Self;
}

impl Splat for f32 {
    type T = Self;

    fn splat(value: Self::T) -> Self {
        value
    }
}

impl Splat for f64 {
    type T = Self;

    fn splat(value: Self::T) -> Self {
        value
    }
}

impl Splat for Vec2 {
    type T = f32;

    fn splat(value: Self::T) -> Self {
        Vec2::splat(value)
    }
}

impl Splat for Vec3 {
    type T = f32;

    fn splat(value: Self::T) -> Self {
        Vec3::splat(value)
    }
}

impl Splat for Vec4 {
    type T = f32;

    fn splat(value: Self::T) -> Self {
        Vec4::splat(value)
    }
}

impl Splat for DVec2 {
    type T = f64;

    fn splat(value: Self::T) -> Self {
        DVec2::splat(value)
    }
}

impl Splat for DVec3 {
    type T = f64;

    fn splat(value: Self::T) -> Self {
        DVec3::splat(value)
    }
}

impl Splat for DVec4 {
    type T = f64;

    fn splat(value: Self::T) -> Self {
        DVec4::splat(value)
    }
}
