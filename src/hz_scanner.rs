use constants::*;
use scale;
use transform::ftransform;
use util;

pub struct HzScanner<'a, F>
  where F: Fn(f64) -> f64 + 'a {
  hz: f64,
  end_hz: f64,
  f: &'a F,
  t: f64,
}

#[derive(Debug)]
pub struct DetectedHz {
  pub hz: f64,
  pub amplitude: f64,
}

impl<'a, F> HzScanner<'a, F>
  where F: Fn(f64) -> f64 + 'a {
  pub fn new(f: &'a F, start_hz: f64, end_hz: f64, t: f64) -> HzScanner<'a, F> {
    HzScanner {
      hz: start_hz,
      end_hz,
      f,
      t,
    }
  }
}

impl<'a, F> Iterator for HzScanner<'a, F>
  where F: Fn(f64) -> f64 + 'a {
  type Item = DetectedHz;

  fn next(&mut self) -> Option<Self::Item> {
    while self.hz <= self.end_hz {
      let hz = self.hz;
      self.hz += SCAN_HZ_RESOLUTION;

      let coeffs = ftransform(hz, self.f, self.t);
      let amplitude = util::amplitude(coeffs);
      if amplitude > SCAN_THRESHOLD {
        return Some(DetectedHz {
          hz,
          amplitude,
        });
      }
    }

    None
  }
}
