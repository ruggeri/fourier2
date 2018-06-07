use constants;
use pitch::Pitch;
use transform::ftransform;
use util;

pub fn scale_for_octave(octave: i32) -> Vec<Pitch> {
  vec![
    format!("A.{}", octave).parse().unwrap(),
    format!("A#{}", octave).parse().unwrap(),
    format!("B.{}", octave).parse().unwrap(),
    format!("C.{}", octave).parse().unwrap(),
    format!("C#{}", octave).parse().unwrap(),
    format!("D.{}", octave).parse().unwrap(),
    format!("D#{}", octave).parse().unwrap(),
    format!("E.{}", octave).parse().unwrap(),
    format!("F.{}", octave).parse().unwrap(),
    format!("F#{}", octave).parse().unwrap(),
    format!("G.{}", octave).parse().unwrap(),
    format!("G#{}", octave).parse().unwrap(),
  ]
}

pub fn piano_pitches() -> Vec<Pitch> {
  let mut pitches: Vec<Pitch> = Vec::new();
  for octave in 0..8 {
    pitches.extend(scale_for_octave(octave));
  }

  pitches
}

pub fn scan<F>(f: F, t: f64) -> Vec<Pitch>
  where F: Fn(f64) -> f64 {
  let mut detected_pitches = Vec::<Pitch>::new();

  for p in piano_pitches() {
    let hz = p.freq();
    let coeffs = ftransform(hz as f64, |t| f(t), t);
    let amplitude = util::amplitude(coeffs);
    if amplitude > constants::SCAN_THRESHOLD {
      detected_pitches.push(p);
    }
  }

  detected_pitches
}
