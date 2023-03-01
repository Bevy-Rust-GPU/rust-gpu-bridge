//! WGSL `log()`

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Equivalent of the WGSL `log()` function.
///
/// Computes the natural logarithm of the provided value.
pub trait NaturalLog {
    const BASE: f32 = 2.718281828459;

    fn natural_log(&self) -> Self;
}

impl NaturalLog for f32 {
    fn natural_log(&self) -> Self {
        self.log(Self::BASE)
    }
}

