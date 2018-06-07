use std::str::FromStr;

#[derive(Debug)]
pub enum PitchClass {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
}

use PitchClass::*;

impl PitchClass {
  pub fn semitones_above_a(&self) -> i32 {
    match self {
      A => 0,
      B => 2,
      C => 3,
      D => 5,
      E => 7,
      F => 8,
      G => 10,
    }
  }

  pub fn base_freq(&self) -> f64 {
    440.0 * (2.0_f64).powf(self.semitones_above_a() as f64 / 12.0)
  }
}

#[derive(Debug)]
pub enum PitchClassParseErr {
  InvalidLength,
  UnknownPitchClass
}

use self::PitchClassParseErr::*;

impl FromStr for PitchClass {
  type Err = PitchClassParseErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 1 {
      return Err(InvalidLength);
    }

    match s {
      "A" => Ok(A),
      "B" => Ok(B),
      "C" => Ok(C),
      "D" => Ok(D),
      "E" => Ok(E),
      "F" => Ok(F),
      "G" => Ok(G),
      _ => Err(UnknownPitchClass),
    }
  }
}
