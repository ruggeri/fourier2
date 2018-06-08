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

static mut _PPS_OPTION: Option<Vec<Pitch>> = None;

pub fn piano_pitches() -> &'static Vec<Pitch> {
  unsafe {
    if let Some(ref pps) = _PPS_OPTION {
      return pps;
    }

    let mut pps: Vec<Pitch> = Vec::new();
    for octave in 0..8 {
      pps.extend(scale_for_octave(octave));
    }

    _PPS_OPTION = Some(pps);
    piano_pitches()
  }
}
