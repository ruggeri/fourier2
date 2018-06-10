use constants::*;
use super::note::Note;

pub struct SongIterator<'a> {
  notes: &'a Vec<Note>,
  note_start_idx: usize,
  t: f64,
  dt: f64,
}

impl<'a> SongIterator<'a> {
  pub fn new(notes: &'a Vec<Note>) -> SongIterator<'a> {
    SongIterator {
      notes,
      note_start_idx: 0,
      t: 0.0,
      dt: 1.0_f64 / SAMPLE_RATE,
    }
  }
}

impl<'a> Iterator for SongIterator<'a> {
  type Item = f64;

  fn next(&mut self) -> Option<Self::Item> {
    if self.notes.last().unwrap().end_time < self.t {
      return None;
    }

    let mut val = 0.0_f64;
    let mut note_idx = self.note_start_idx;

    while note_idx < self.notes.len() {
      let note = &self.notes[note_idx];
      if note.end_time < self.t {
        self.note_start_idx = note_idx;
        note_idx += 1;
        continue;
      }
      if self.t < note.start_time {
        break;
      }

      val += note.val(self.t);

      note_idx += 1;
    }

    self.t += self.dt;

    Some(val)
  }
}
