use crate::glam::{Vec2, Vec3, Vec4};

pub trait Splat {
    fn splat(value: f32) -> Self;
}

impl Splat for f32 {
    fn splat(value: f32) -> Self {
        value
    }
}

impl Splat for Vec2 {
    fn splat(value: f32) -> Self {
        Vec2::splat(value)
    }
}

impl Splat for Vec3 {
    fn splat(value: f32) -> Self {
        Vec3::splat(value)
    }
}

impl Splat for Vec4 {
    fn splat(value: f32) -> Self {
        Vec4::splat(value)
    }
}

