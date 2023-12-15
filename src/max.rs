//! Shader `max()`

/// Returns the greater of two values
pub trait Max {
    fn max(self, rhs: Self) -> Self;
}

impl Max for u32 {
    fn max(self, rhs: Self) -> Self {
        Ord::max(self, rhs)
    }
}

impl Max for u64 {
    fn max(self, rhs: Self) -> Self {
        Ord::max(self, rhs)
    }
}

impl Max for i32 {
    fn max(self, rhs: Self) -> Self {
        Ord::max(self, rhs)
    }
}

impl Max for i64 {
    fn max(self, rhs: Self) -> Self {
        Ord::max(self, rhs)
    }
}


impl Max for f32 {
    fn max(self, rhs: Self) -> Self {
        #[cfg(feature = "glam")]
        {
            f32::max(self, rhs)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::max(self, rhs)
        }
    }
}

impl Max for f64 {
    fn max(self, rhs: Self) -> Self {
        #[cfg(feature = "glam")]
        {
            f64::max(self, rhs)
        }

        #[cfg(feature = "spirv-std")]
        {
            spirv_std::num_traits::Float::max(self, rhs)
        }
    }
}

