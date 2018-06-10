use super::pitch_class::{PitchClass, PitchClassParseErr};
use std::fmt;
use std::str::FromStr;
use util;

#[derive(Clone, Copy)]
pub struct Pitch {
    pub pitch_class: PitchClass,
    pub sharp: bool,
    pub octave: i32,
    pub hz: f64,
}

fn hz(pitch_class: PitchClass, sharp: bool, octave: i32) -> f64 {
    let mut hz: f64 = pitch_class.base_freq();

    if sharp {
        hz *= (2.0_f64).powf(1.0 / 12.0);
    }

    hz *= (2.0_f64).powf(f64::from(octave - 4));

    hz
}

impl Pitch {
    pub fn new(pitch_class: PitchClass, sharp: bool, octave: i32) -> Pitch {
        Pitch {
            pitch_class,
            sharp,
            octave,
            hz: hz(pitch_class, sharp, octave),
        }
    }

    pub fn val(&self, t: f64) -> f64 {
        util::sin_val_for_freq_at_time(self.hz, t)
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
            Err(err) => {
                return Err(PitchParseErr(err));
            }
        };
        let sharp = match &s[1..2] {
            "." => false,
            "#" => true,
            _ => {
                return Err(InvalidSharpSymbol);
            }
        };

        let octave: i32 = match (&s[2..]).parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(OctaveParseFailed);
            }
        };

        Ok(Pitch::new(pitch_class, sharp, octave))
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let accidental_symbol = if self.sharp { "#" } else { "." };

        write!(
            f,
            "{}{}{}",
            self.pitch_class, accidental_symbol, self.octave
        )
    }
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
