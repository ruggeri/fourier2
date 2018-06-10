use constants::*;
use util;

#[derive(Builder, Clone, Copy)]
pub struct FourierTransformOpts {
  #[builder(default = "FOURIER_WINDOW_WIDTH")]
  pub window_width: f64,
  #[builder(default = "FOURIER_SAMPLE_RATE_PER_SEC")]
  pub sample_rate_per_sec: f64,
}

pub fn ftransform<F>(freq: f64, fun: F, window_center: f64, opts: FourierTransformOpts) -> (f64, f64)
  where F: Fn(f64) -> f64 {
  let start = (window_center - opts.window_width).max(0.0_f64);
  let end = window_center + (window_center - start);
  let total_width = end - start;
  let num_samples = total_width * opts.sample_rate_per_sec;

  let mut sin_amplitude = 0.0_f64;
  let mut cos_amplitude = 0.0_f64;

  let mut t = start;
  while t <= end {
    let fun_val = fun(t);
    sin_amplitude += fun_val * util::sin_val_for_freq_at_time(freq, t);
    cos_amplitude += fun_val * util::cos_val_for_freq_at_time(freq, t);

    t += 1.0_f64 / FOURIER_SAMPLE_RATE_PER_SEC;
  }

  // Effectively divides by the period.
  sin_amplitude /= num_samples;
  cos_amplitude /= num_samples;

  // But we want to multiply by 2/T, so here's the x2 part.
  sin_amplitude *= 2.0_f64;
  cos_amplitude *= 2.0_f64;

  (sin_amplitude, cos_amplitude)
}
