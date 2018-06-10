use constants::*;
use super::opts::HzScannerOpts;
use transform::{FourierTransformOpts, ftransform};
use util;

pub struct HzScanner<F>
  where F: Fn(f64) -> f64 + Copy {
  f: F,
  t: f64,
  hz: f64,
  opts: HzScannerOpts,
}

#[derive(Debug)]
pub struct DetectedHz {
  pub hz: f64,
  pub amplitude: f64,
  pub time: f64,
}

impl<F> HzScanner<F>
  where F: Fn(f64) -> f64 + Copy {
  pub fn new(f: F, t: f64, opts: HzScannerOpts) -> HzScanner<F> {
    HzScanner {
      f,
      t,
      hz: opts.start_hz,
      opts,
    }
  }

  pub fn scan(f: F, start_t: f64, duration: f64, opts: HzScannerOpts) -> impl Iterator<Item=DetectedHz> {
    let samples = (duration / opts.scan_time_resolution) as usize;
    (0..samples).flat_map(move |idx| {
      let t = start_t + (idx as f64) * opts.scan_time_resolution;
      HzScanner::new(f, t, opts)
    })
  }
}

impl<F> Iterator for HzScanner<F>
  where F: Fn(f64) -> f64 + Copy {
  type Item = DetectedHz;

  fn next(&mut self) -> Option<Self::Item> {
    while self.hz <= self.opts.end_hz {
      let hz = self.hz;
      self.hz += SCAN_HZ_RESOLUTION;

      let coeffs = ftransform(hz, self.f, self.t, FourierTransformOpts::from(self.opts));
      let amplitude = util::amplitude(coeffs);
      if amplitude > SCAN_AMPLITUDE_THRESHOLD {
        return Some(DetectedHz {
          hz,
          amplitude,
          time: self.t,
        });
      }
    }

    None
  }
}
