//! Shader `min()`

/// Returns the lesser of two values
pub trait Min {
    fn min(self, rhs: Self) -> Self;
}

impl Min for u32 {
    fn min(self, rhs: Self) -> Self {
        <u32 as Ord>::min(self, rhs)
    }
}

impl Min for u64 {
    fn min(self, rhs: Self) -> Self {
        <u64 as Ord>::min(self, rhs)
    }
}

impl Min for i32 {
    fn min(self, rhs: Self) -> Self {
        <i32 as Ord>::max(self, rhs)
    }
}

impl Min for i64 {
    fn min(self, rhs: Self) -> Self {
        <i64 as Ord>::max(self, rhs)
    }
}

impl Min for f32 {
    fn min(self, rhs: Self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::min(self, rhs)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::min(self, rhs)
        }
    }
}

impl Min for f64 {
    fn min(self, rhs: Self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::min(self, rhs)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::min(self, rhs)
        }
    }
}
