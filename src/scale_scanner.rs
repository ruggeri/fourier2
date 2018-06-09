use constants::*;
use pitch::Pitch;
use scale;
use transform::ftransform;
use util;

pub struct ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + Copy {
  piano_pitches: <&'a Vec<Pitch> as IntoIterator>::IntoIter,
  f: F,
  t: f64,
}

#[derive(Debug)]
pub struct DetectedPitch {
  pub pitch: Pitch,
  pub amplitude: f64,
  pub time: f64,
}

impl<'a, F> ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + Copy {
  pub fn new(f: F, t: f64) -> ScaleScanner<'a, F> {
    ScaleScanner {
      piano_pitches: scale::piano_pitches().iter(),
      f,
      t,
    }
  }

  pub fn scan(f: F, start_t: f64, duration: f64) -> impl Iterator<Item=DetectedPitch> {
    let samples = (duration / SCAN_TIME_RESOLUTION) as usize;
    (0..samples).flat_map(move |idx| {
      let t = start_t + (idx as f64) * SCAN_TIME_RESOLUTION;
      ScaleScanner::new(f, t)
    })
  }
}

impl<'a, F> Iterator for ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + Copy {
  type Item = DetectedPitch;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(&pitch) = self.piano_pitches.next() {
      let coeffs = ftransform(pitch.hz as f64, self.f, self.t);
      let amplitude = util::amplitude(coeffs);
      if amplitude > SCAN_THRESHOLD {
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
