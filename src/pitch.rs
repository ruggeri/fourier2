use pitch_class::{PitchClass, PitchClassParseErr};
use std::fmt;
use std::str::FromStr;
use util;

#[derive(Clone, Copy)]
pub struct Pitch {
  pitch_class: PitchClass,
  sharp: bool,
  octave: i32,
}

impl Pitch {
  pub fn freq(&self) -> f64 {
    let mut f: f64 = self.pitch_class.base_freq();

    if self.sharp {
      f *= (2.0_f64).powf(1.0 / 12.0);
    }

    f *= (2.0_f64).powf((self.octave - 4) as f64);

    f
  }

  pub fn val(&self, t: f64) -> f64 {
    util::sin_val_for_freq_at_time(self.freq(), t)
  }
}

#[derive(Debug)]
pub enum PitchParseErr {
  InvalidSharpSymbol,
  LengthError,
  OctaveParseFailed,
  PitchParseErr(PitchClassParseErr),
}

use self::PitchParseErr::*;

impl FromStr for Pitch {
  type Err = PitchParseErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 3 {
      return Err(LengthError);
    }

    let pitch_class: PitchClass = match (&s[0..1]).parse() {
      Ok(pc) => pc,
      Err(err) => { return Err(PitchParseErr(err)); },
    };
    let sharp = match &s[1..2] {
      "." => false,
      "#" => true,
      _ => { return Err(InvalidSharpSymbol); },
    };

    let octave: i32 = match (&s[2..]).parse() {
      Ok(n) => n,
      Err(_) => { return Err(OctaveParseFailed); },
    };

    Ok(Pitch {
      pitch_class,
      sharp,
      octave,
    })
  }
}

impl fmt::Display for Pitch {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let accidental_symbol = if self.sharp { "#" } else { "." };

    write!(f, "{}{}{}", self.pitch_class, accidental_symbol, self.octave)
  }
}

impl fmt::Debug for Pitch {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}
