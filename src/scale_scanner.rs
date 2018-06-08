use constants::*;
use pitch::Pitch;
use scale;
use transform::ftransform;
use util;

pub struct ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + 'a {
  piano_pitches: <&'a Vec<Pitch> as IntoIterator>::IntoIter,
  f: &'a F,
  t: f64,
}

#[derive(Debug)]
pub struct DetectedPitch {
  pub pitch: Pitch,
  pub amplitude: f64,
}

impl<'a, F> ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + 'a {
  pub fn new(f: &'a F, t: f64) -> ScaleScanner<'a, F> {
    ScaleScanner {
      piano_pitches: scale::piano_pitches().iter(),
      f,
      t,
    }
  }
}

impl<'a, F> Iterator for ScaleScanner<'a, F>
  where F: Fn(f64) -> f64 + 'a {
  type Item = DetectedPitch;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(&pitch) = self.piano_pitches.next() {
      let hz = pitch.freq();
      let coeffs = ftransform(hz as f64, self.f, self.t);
      let amplitude = util::amplitude(coeffs);
      if amplitude > SCAN_THRESHOLD {
        return Some(DetectedPitch {
          pitch,
          amplitude,
        });
      }
    }

    None
  }
}
