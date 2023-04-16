pub use alloc::{
    format,
    string::{String, ToString},
};

use crate::glam::{Vec2, Vec3, Vec4};

pub trait Named {
    fn short_name() -> String;

    fn module() -> String {
        Default::default()
    }

    fn name() -> String {
        let module = Self::module();
        let short_name = Self::short_name();
        if module.is_empty() {
            short_name
        } else {
            module + "::" + &short_name
        }
    }
}

impl Named for f32 {
    fn short_name() -> String {
        "f32".to_string()
    }
}

impl Named for Vec2 {
    fn module() -> String {
        "rust_gpu_bridge::glam".to_string()
    }

    fn short_name() -> String {
        "Vec2".to_string()
    }
}

impl Named for Vec3 {
    fn module() -> String {
        "rust_gpu_bridge::glam".to_string()
    }

    fn short_name() -> String {
        "Vec3".to_string()
    }
}

impl Named for Vec4 {
    fn module() -> String {
        "rust_gpu_bridge::glam".to_string()
    }

    fn short_name() -> String {
        "Vec4".to_string()
    }
}

impl<LHS, RHS> Named for (LHS, RHS)
where
    LHS: Named,
    RHS: Named,
{
    fn short_name() -> String {
        format!("({}, {})", LHS::short_name(), RHS::short_name())
    }

    fn name() -> String {
        format!("({}, {})", LHS::name(), RHS::name())
    }
}

impl Named for ()
{
    fn short_name() -> String {
        "()".to_string()
    }
}
