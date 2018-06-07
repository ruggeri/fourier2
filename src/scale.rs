use pitch::Pitch;

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
