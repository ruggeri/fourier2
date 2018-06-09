use constants::*;
use transform::ftransform;
use util;

pub struct HzScanner<F>
  where F: Fn(f64) -> f64 + Copy {
  hz: f64,
  end_hz: f64,
  f: F,
  t: f64,
}

#[derive(Debug)]
pub struct DetectedHz {
  pub hz: f64,
  pub amplitude: f64,
  pub time: f64,
}

impl<F> HzScanner<F>
  where F: Fn(f64) -> f64 + Copy {
  pub fn new(f: F, start_hz: f64, end_hz: f64, t: f64) -> HzScanner<F> {
    HzScanner {
      hz: start_hz,
      end_hz,
      f,
      t,
    }
  }

  pub fn scan(f: F, start_hz: f64, end_hz: f64, start_t: f64, duration: f64) -> impl Iterator<Item=DetectedHz> {
    let samples = (duration / SCAN_TIME_RESOLUTION) as usize;
    (0..samples).flat_map(move |idx| {
      let t = start_t + (idx as f64) * SCAN_TIME_RESOLUTION;
      HzScanner::new(f, start_hz, end_hz, t)
    })
  }
}

impl<F> Iterator for HzScanner<F>
  where F: Fn(f64) -> f64 + Copy {
  type Item = DetectedHz;

  fn next(&mut self) -> Option<Self::Item> {
    while self.hz <= self.end_hz {
      let hz = self.hz;
      self.hz += SCAN_HZ_RESOLUTION;

      let coeffs = ftransform(hz, self.f, self.t);
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
