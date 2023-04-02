/*
    TODO's:
    - implement pitchshift on taps_output
    - implement reverse on predelay
*/

include!(concat!(env!("OUT_DIR"), "/wave_table.rs"));
mod allpass_filter;
mod biquad_filter;
mod dc_block;
mod delay_line;
mod delta;
mod envelope_follower;
mod float_ext;
mod grains;
mod lfo;
mod mix;
mod one_pole_filter;
mod pan;
mod phasor;
mod reverb;
mod shimmer;
mod slide;
mod tap;
mod taps;
mod tilt_filter;
mod wave_table;

pub const MIN_SIZE: f32 = 1.;
pub const MAX_SIZE: f32 = 500.;
pub const MAX_DEPTH: f32 = 3.;
pub use self::reverb::Reverb;
