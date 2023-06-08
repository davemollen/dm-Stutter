#![feature(portable_simd)]
mod allpass_filter;
mod average;
mod biquad_filter;
mod dc_block;
mod delay_line;
mod delta;
mod early_reflections;
mod float_ext;
mod grains;
mod mix;
mod one_pole_filter;
mod phasor;
mod reverb;
mod reverse;
mod saturation_activator;
mod shimmer;
mod smooth_parameters;
mod tap;
mod taps;
mod tilt_filter;

pub const MIN_PREDELAY: f32 = 7.;
pub const MAX_PREDELAY: f32 = 500.;
pub const MIN_SIZE: f32 = 1.;
pub const MAX_SIZE: f32 = 500.;
pub const MAX_DEPTH: f32 = 3.;
pub use self::reverb::Reverb;
