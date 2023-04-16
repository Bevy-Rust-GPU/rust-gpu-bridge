use crate::glam::{Vec2, Vec3, Vec4};

pub trait ToVec<V> {
    fn to_vec(self) -> V;
}

impl ToVec<f32> for f32 {
    fn to_vec(self) -> f32 {
        self
    }
}

impl ToVec<Vec2> for f32 {
    fn to_vec(self) -> Vec2 {
        Vec2::splat(self)
    }
}

impl ToVec<Vec3> for f32 {
    fn to_vec(self) -> Vec3 {
        Vec3::splat(self)
    }
}

impl ToVec<Vec4> for f32 {
    fn to_vec(self) -> Vec4 {
        Vec4::splat(self)
    }
}

impl ToVec<f32> for Vec2 {
    fn to_vec(self) -> f32 {
        self.x
    }
}

impl ToVec<Vec2> for Vec2 {
    fn to_vec(self) -> Vec2 {
        self
    }
}

impl ToVec<Vec3> for Vec2 {
    fn to_vec(self) -> Vec3 {
        self.extend(0.0)
    }
}

impl ToVec<Vec4> for Vec2 {
    fn to_vec(self) -> Vec4 {
        self.extend(0.0).extend(0.0)
    }
}

impl ToVec<f32> for Vec3 {
    fn to_vec(self) -> f32 {
        self.x
    }
}

impl ToVec<Vec2> for Vec3 {
    fn to_vec(self) -> Vec2 {
        self.truncate()
    }
}

impl ToVec<Vec3> for Vec3 {
    fn to_vec(self) -> Vec3 {
        self
    }
}

impl ToVec<Vec4> for Vec3 {
    fn to_vec(self) -> Vec4 {
        self.extend(0.0)
    }
}

impl ToVec<f32> for Vec4 {
    fn to_vec(self) -> f32 {
        self.x
    }
}

impl ToVec<Vec2> for Vec4 {
    fn to_vec(self) -> Vec2 {
        self.truncate().truncate()
    }
}

impl ToVec<Vec3> for Vec4 {
    fn to_vec(self) -> Vec3 {
        self.truncate()
    }
}

impl ToVec<Vec4> for Vec4 {
    fn to_vec(self) -> Vec4 {
        self
    }
}
