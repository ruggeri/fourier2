use super::pitch::Pitch;

pub struct Chord {
    pitches: Vec<Pitch>,
    start_time: f64,
    end_time: f64,
    amplitude: f64,
}

impl Chord {
    pub fn new(pitches: Vec<Pitch>, start_time: f64, duration: f64, amplitude: f64) -> Chord {
        Chord {
            pitches,
            start_time,
            end_time: start_time + duration,
            amplitude,
        }
    }

    pub fn val(&self, t: f64) -> f64 {
        if t < self.start_time {
            return 0.0;
        } else if self.end_time < t {
            return 0.0;
        }

        let mut val = 0.0;
        for pitch in &self.pitches {
            val += pitch.val(t) * self.amplitude;
        }

        val
    }
}
