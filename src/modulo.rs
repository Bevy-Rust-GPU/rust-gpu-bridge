use crate::glam::{Vec2, Vec3, Vec4};

pub trait Mod {
    fn modulo(self, modulus: Self) -> Self;
}

impl Mod for f32 {
    fn modulo(self, modulus: Self) -> Self {
        self.rem_euclid(modulus)
    }
}

impl Mod for Vec2 {
    fn modulo(self, modulus: Self) -> Self {
        Vec2::new(self.x.modulo(modulus.x), self.y.modulo(modulus.y))
    }
}

impl Mod for Vec3 {
    fn modulo(self, modulus: Self) -> Self {
        Vec3::new(
            self.x.modulo(modulus.x),
            self.y.modulo(modulus.y),
            self.z.modulo(modulus.z),
        )
    }
}

impl Mod for Vec4 {
    fn modulo(self, modulus: Self) -> Self {
        Vec4::new(
            self.x.modulo(modulus.x),
            self.y.modulo(modulus.y),
            self.z.modulo(modulus.z),
            self.w.modulo(modulus.w),
        )
    }
}

