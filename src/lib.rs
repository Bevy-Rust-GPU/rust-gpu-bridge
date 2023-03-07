//! # rust-gpu-bridge
//!
//! Bridge crate for writing `no-std` code that can be used in both `rust-gpu` and regular Rust.
//! 
//! Conditionally gates `glam` and `spirv-std::glam` behind cargo features,
//! and Contains utility traits for replicating common shading language functions.

#![no_std]

#[cfg(feature = "glam")]
pub use glam;

#[cfg(feature = "spirv-std")]
pub use spirv_std::glam;

#[cfg(all(not(feature = "glam"), not(feature = "spirv-std")))]
compile_error!("Either the glam or spirv-std feature must be enabled.");

pub mod reflect;
pub mod saturate;
pub mod smooth_step;
pub mod natural_log;
pub mod mix;

pub mod prelude;

use glam::Vec3;

#[cfg(feature = "spirv-std")]
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

/// Convert from HSV to RGB
///
/// From `bevy_pbr/src/render/utils.wgsl`
pub fn hsv2rgb(hue: f32, saturation: f32, value: f32) -> Vec3 {
    let rgb = ((((hue * 6.0 + Vec3::new(0.0, 4.0, 2.0)) % 6.0) - 3.0).abs() - 1.0)
        .clamp(Vec3::ZERO, Vec3::ONE);

    Vec3::ONE.lerp(rgb, saturation) * value
}

/// Generate a random floating point number
///
/// From `bevy_pbr/src/render/utils.wgsl`
pub fn random_1d(s: f32) -> f32 {
    return ((s * 12.9898).sin() * 43758.5453123).fract();
}
