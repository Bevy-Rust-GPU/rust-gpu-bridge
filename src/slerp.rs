use crate::{
    glam::{Vec2, Vec3, Vec4},
    Mix,
};

pub trait Slerp {
    fn slerp(self, to: Self, t: f32) -> Self;
}

impl Slerp for f32 {
    fn slerp(self, to: Self, t: f32) -> f32 {
        self.mix(to, t)
    }
}

impl Slerp for Vec2 {
    fn slerp(self, to: Self, t: f32) -> Vec2 {
        Vec2::slerp(self, to, t)
    }
}

impl Slerp for Vec3 {
    fn slerp(self, to: Self, t: f32) -> Vec3 {
        Vec3::slerp(self, to, t)
    }
}

impl Slerp for Vec4 {
    fn slerp(self, to: Self, t: f32) -> Vec4 {
        Vec4::slerp(self, to, t)
    }
}

