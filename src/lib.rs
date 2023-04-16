//! # rust-gpu-bridge
//!
//! Bridge crate for writing `no-std` code that can be used in both `rust-gpu` and regular Rust.
//!
//! Conditionally gates `glam` and `spirv-std::glam` behind cargo features,
//! and contains utility traits for replicating common shading language functions.

#![no_std]

#[cfg(all(not(feature = "glam"), not(feature = "spirv-std")))]
compile_error!("Either the glam or spirv-std feature must be enabled.");

#[cfg(feature = "glam")]
pub use rust_gpu_bridge_macros::Named;

#[cfg(feature = "glam")]
#[macro_use]
extern crate alloc;

#[cfg(feature = "glam")]
pub use glam;

#[cfg(feature = "spirv-std")]
pub use spirv_std::glam;

#[cfg(feature = "spirv-std")]
pub use spirv_std::num_traits;

#[cfg(all(feature = "glam", feature = "spirv-std"))]
compile_error!("Features glam and spirv-std may not be enabled simultaneously.");

mod abs;
mod acos;
mod as_vec2;
mod asin;
mod atan2;
mod clamp;
mod cos;
mod cross;
mod dot;
mod exp2;
mod fract;
mod length;
mod is_normalized;
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
mod splat;
mod sqlen;
mod sqrt;
mod step;
mod tan;

#[cfg(feature = "glam")]
mod named;

pub use abs::*;
pub use acos::*;
pub use as_vec2::*;
pub use asin::*;
pub use atan2::*;
pub use clamp::*;
pub use cos::*;
pub use cross::*;
pub use dot::*;
pub use exp2::*;
pub use fract::*;
pub use length::*;
pub use log2::*;
pub use mix::*;
pub use is_normalized::*;
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
pub use splat::*;
pub use sqlen::*;
pub use sqrt::*;
pub use step::*;
pub use tan::*;

#[cfg(feature = "glam")]
pub use named::*;

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
