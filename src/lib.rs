//! # rust-gpu-bridge
//!
//! Bridge crate for writing `no-std` code that can be used in both `rust-gpu` and regular Rust.
//!
//! Conditionally gates `glam` and `spirv-std::glam` behind cargo features,
//! and contains utility traits for replicating common shading language functions.

#![no_std]

#[cfg(feature = "glam")]
pub use glam;

#[cfg(feature = "spirv-std")]
pub use spirv_std::glam;

#[cfg(all(not(feature = "glam"), not(feature = "spirv-std")))]
compile_error!("Either the glam or spirv-std feature must be enabled.");

mod abs;
mod acos;
mod asin;
mod atan2;
mod clamp;
mod cos;
mod dot;
mod exp2;
mod fract;
mod length;
mod log2;
mod mix;
mod modulo;
mod natural_log;
mod normalize;
mod pow;
mod reflect;
mod round;
mod saturate;
mod sign;
mod sin;
mod smooth_step;
mod sqrt;
mod step;
mod tan;

pub use abs::*;
pub use acos::*;
pub use asin::*;
pub use atan2::*;
pub use clamp::*;
pub use cos::*;
pub use dot::*;
pub use exp2::*;
pub use fract::*;
pub use length::*;
pub use log2::*;
pub use mix::*;
pub use modulo::*;
pub use natural_log::*;
pub use normalize::*;
pub use pow::*;
pub use reflect::*;
pub use round::*;
pub use saturate::*;
pub use sign::*;
pub use sin::*;
pub use smooth_step::*;
pub use sqrt::*;
pub use step::*;
pub use tan::*;

use glam::Vec3;

/// Convert from HSV to RGB.
///
/// From `bevy_pbr/src/render/utils.wgsl`
pub fn hsv2rgb(hue: f32, saturation: f32, value: f32) -> Vec3 {
    let rgb = ((((hue * 6.0 + Vec3::new(0.0, 4.0, 2.0)) % 6.0) - 3.0).abs() - 1.0)
        .clamp(Vec3::ZERO, Vec3::ONE);

    Vec3::ONE.lerp(rgb, saturation) * value
}

/// Generate a random floating point number.
///
/// From `bevy_pbr/src/render/utils.wgsl`
pub fn random_1d(s: f32) -> f32 {
    return ((s * 12.9898).sin() * 43758.5453123).fract();
}
