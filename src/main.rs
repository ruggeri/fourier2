#![feature(label_break_value)]

extern crate byteorder;
extern crate fourier2;
extern crate rand;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

use fourier2::{constants::*, Note, PCMFile, ScaleScanner, SongIterator, util};
use rand::{
  distributions::Normal,
  prelude::*,
};
use std::fs::File;


fn main() {
  let mut notes = vec![];

  // notes.extend(vec![
  //   Note::new("A.3".parse().unwrap(), 0.00, 1.00, 0.1),
  //   Note::new("C.3".parse().unwrap(), 0.25, 0.75, 0.1),
  //   Note::new("E.3".parse().unwrap(), 0.50, 0.50, 0.1),
  //   Note::new("A.4".parse().unwrap(), 0.75, 0.25, 0.1),
  // ]);

  // notes.extend(vec![
  //   Note::new("F.2".parse().unwrap(), 1.00, 1.00, 0.1),
  //   Note::new("A.3".parse().unwrap(), 1.25, 0.75, 0.1),
  //   Note::new("C.3".parse().unwrap(), 1.50, 0.50, 0.1),
  //   Note::new("F.3".parse().unwrap(), 1.75, 0.25, 0.1),
  // ]);

  // notes.extend(vec![
  //   Note::new("D.2".parse().unwrap(), 2.00, 1.00, 0.1),
  //   Note::new("F.2".parse().unwrap(), 2.25, 0.75, 0.1),
  //   Note::new("A.3".parse().unwrap(), 2.50, 0.50, 0.1),
  //   Note::new("D.3".parse().unwrap(), 2.75, 0.25, 0.1),
  // ]);

  // notes.extend(vec![
  //   Note::new("G.1".parse().unwrap(), 3.00, 2.00, 0.1),
  //   Note::new("B.2".parse().unwrap(), 3.25, 1.75, 0.1),
  //   Note::new("D.2".parse().unwrap(), 3.50, 1.50, 0.1),
  //   Note::new("G.2".parse().unwrap(), 3.75, 0.25, 0.1),
  //   Note::new("F.2".parse().unwrap(), 4.00, 1.00, 0.1),
  // ]);

  // let dist = Normal::new(0.0_f64, 1.0_f64);

  println!("Reading PCM input file!");
  let file = PCMFile::open("./rcar.pcm");

  let f = |t| {
    let idx = (t * SAMPLE_RATE) as usize;
    util::i16_to_f64(file.i16s[idx])
  };

  println!("Searching for freqs!");

  let mut t = 0.0;
  while t < 60.0 {
    for detected_pitch in ScaleScanner::new(&f, t) {
      println!("{}: {:?}", t, detected_pitch);
      notes.push(Note::new(detected_pitch.pitch, t, SCAN_TIME_RESOLUTION, detected_pitch.amplitude));
    }

    t += SCAN_TIME_RESOLUTION;
  }

  println!("Writing PCM output file!");
  let mut file = File::create("./samples.pcm").unwrap();
  util::play_to_file(&mut file, SongIterator::new(&notes));
}
