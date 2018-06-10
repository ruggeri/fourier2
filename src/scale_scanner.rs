use constants::*;
use pitch::Pitch;
use scale;
use transform::{FourierTransformOpts, FourierTransformOptsBuilder, ftransform};
use util;

#[derive(Builder, Clone, Copy)]
pub struct ScaleScannerOpts {
  #[builder(default = "SCAN_TIME_RESOLUTION")]
  pub scan_time_resolution: f64,
  #[builder(default = "FOURIER_WINDOW_WIDTH")]
  pub window_width: f64,
  #[builder(default = "FOURIER_SAMPLE_RATE_PER_SEC")]
  pub sample_rate_per_sec: f64,
}

impl From<ScaleScannerOpts> for FourierTransformOpts {
  fn from(opts: ScaleScannerOpts) -> FourierTransformOpts {
    FourierTransformOptsBuilder
      ::default()
      .window_width(opts.window_width)
      .sample_rate_per_sec(opts.sample_rate_per_sec)
      .build().unwrap()
  }
}

pub struct ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + Copy {
  piano_pitches: <&'a Vec<Pitch> as IntoIterator>::IntoIter,
  f: F,
  t: f64,
  opts: ScaleScannerOpts,
}

#[derive(Debug)]
pub struct DetectedPitch {
  pub pitch: Pitch,
  pub amplitude: f64,
  pub time: f64,
}

impl<'a, F> ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + Copy {
  pub fn new(f: F, t: f64, opts: ScaleScannerOpts) -> ScaleScanner<'a, F> {
    ScaleScanner {
      piano_pitches: scale::piano_pitches().iter(),
      f,
      t,
      opts,
    }
  }

  pub fn scan(f: F, start_t: f64, duration: f64, opts: ScaleScannerOpts) -> impl Iterator<Item=DetectedPitch> {
    let samples = (duration / opts.scan_time_resolution) as usize;
    (0..samples).flat_map(move |idx| {
      let t = start_t + (idx as f64) * opts.scan_time_resolution;
      ScaleScanner::new(f, t, opts)
    })
  }
}

impl<'a, F> Iterator for ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + Copy {
  type Item = DetectedPitch;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(&pitch) = self.piano_pitches.next() {
      let coeffs = ftransform(pitch.hz as f64, self.f, self.t, FourierTransformOpts::from(self.opts));
      let amplitude = util::amplitude(coeffs);
      if amplitude > SCAN_AMPLITUDE_THRESHOLD {
        return Some(DetectedPitch {
          pitch,
          amplitude,
          time: self.t,
        });
      }
    }

    None
  }
}
