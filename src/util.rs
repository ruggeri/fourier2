use std::f64;
use std::i16;

pub fn sin_val_for_freq_at_time(freq: f64, t: f64) -> f64 {
  let radians = 2.0 * f64::consts::PI * t * freq;
  radians.sin()
}

pub fn cos_val_for_freq_at_time(freq: f64, t: f64) -> f64 {
  let radians = 2.0 * f64::consts::PI * t * freq;
  radians.cos()
}

pub fn i16_to_f64(val: i16) -> f64 {
  (val as f64) / (i16::MAX as f64)
}

pub fn f64_to_i16(val: f64) -> i16 {
  (val * (i16::MAX as f64)) as i16
}

pub fn amplitude(coeffs: (f64, f64)) -> f64 {
  (coeffs.0.powf(2.0_f64) + coeffs.1.powf(2.0_f64)).sqrt()
}
