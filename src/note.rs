use pitch::Pitch;

pub struct Note {
  pitch: Pitch,
  start_time: f64,
  end_time: f64,
  amplitude: f64,
}

impl Note {
  pub fn new(pitch: Pitch, start_time: f64, duration: f64, amplitude: f64) -> Note {
    Note {
      pitch,
      start_time,
      end_time: start_time + duration,
      amplitude,
    }
  }

  pub fn val(&self, t: f64) -> f64 {
    if t < self.start_time {
      0.0
    } else if self.end_time < t {
      0.0
    } else {
      self.pitch.val(t) * self.amplitude
    }
  }
}
