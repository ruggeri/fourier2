use constants::*;
use core::AudioSource;
use util;

#[derive(Builder, Clone, Copy)]
pub struct FourierTransformOpts {
    // How many seconds of samples should we use for short time transform.
    #[builder(default = "DEFAULT_FOURIER_WINDOW_WIDTH")]
    pub window_width: f64,

    // When performing Fourier transform, could "downsample" for speed.
    #[builder(default = "DEFAULT_FOURIER_SAMPLE_RATE")]
    pub sample_rate: f64,
}

pub fn ftransform<'a, AS>(
    freq: f64,
    source: &'a AS,
    window_center: f64,
    opts: FourierTransformOpts,
) -> (f64, f64)
where
    AS: AudioSource,
{
    let start = (window_center - opts.window_width).max(0.0_f64);
    let end = (window_center + (window_center - start)).min(source.duration());
    let total_width = end - start;
    let num_samples = total_width * opts.sample_rate;

    let mut sin_amplitude = 0.0_f64;
    let mut cos_amplitude = 0.0_f64;

    let mut t = start;
    while t <= end {
        let fun_val = source.val_at_time(t);
        sin_amplitude += fun_val * util::sin_val_for_freq_at_time(freq, t);
        cos_amplitude += fun_val * util::cos_val_for_freq_at_time(freq, t);

        t += 1.0_f64 / opts.sample_rate;
    }

    // Effectively divides by the period.
    sin_amplitude /= num_samples;
    cos_amplitude /= num_samples;

    // But we want to multiply by 2/T, so here's the x2 part.
    sin_amplitude *= 2.0_f64;
    cos_amplitude *= 2.0_f64;

    (sin_amplitude, cos_amplitude)
}
