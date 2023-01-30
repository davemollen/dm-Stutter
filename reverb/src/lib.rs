include!(concat!(env!("OUT_DIR"), "/wave_table.rs"));
mod allpass_filter;
mod clip;
mod delay_line;
mod mix;
mod one_pole_filter;
mod reverb;
mod tap;
mod lfo;
mod phasor;
mod taps;

pub use self::reverb::Reverb;
