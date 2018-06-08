#![feature(label_break_value)]

extern crate byteorder;
extern crate fourier2;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use fourier2::{Note, ScaleScanner};

fn play<F>(file: &mut File, f: &F, duration: f64)
  where F: Fn(f64) -> f64 {
  let mut t = 0.0;
  while t < duration {
    let val: f64 = f(t);
    file.write_i16::<LittleEndian>(fourier2::util::f64_to_i16(val)).unwrap();

    t += 1_f64 / fourier2::constants::SAMPLE_RATE;
  }
}

fn main() {
  let mut notes = vec![];

  notes.extend(vec![
    Note::new("A.3".parse().unwrap(), 0.00, 1.00, 0.1),
    Note::new("C.3".parse().unwrap(), 0.25, 0.75, 0.1),
    Note::new("E.3".parse().unwrap(), 0.50, 0.50, 0.1),
    Note::new("A.4".parse().unwrap(), 0.75, 0.25, 0.1),
  ]);

  notes.extend(vec![
    Note::new("F.2".parse().unwrap(), 1.00, 1.00, 0.1),
    Note::new("A.3".parse().unwrap(), 1.25, 0.75, 0.1),
    Note::new("C.3".parse().unwrap(), 1.50, 0.50, 0.1),
    Note::new("F.3".parse().unwrap(), 1.75, 0.25, 0.1),
  ]);

  notes.extend(vec![
    Note::new("D.2".parse().unwrap(), 2.00, 1.00, 0.1),
    Note::new("F.2".parse().unwrap(), 2.25, 0.75, 0.1),
    Note::new("A.3".parse().unwrap(), 2.50, 0.50, 0.1),
    Note::new("D.3".parse().unwrap(), 2.75, 0.25, 0.1),
  ]);

  notes.extend(vec![
    Note::new("G.1".parse().unwrap(), 3.00, 2.00, 0.1),
    Note::new("B.2".parse().unwrap(), 3.25, 1.75, 0.1),
    Note::new("D.2".parse().unwrap(), 3.50, 1.50, 0.1),
    Note::new("G.2".parse().unwrap(), 3.75, 0.25, 0.1),
    Note::new("F.2".parse().unwrap(), 4.00, 1.00, 0.1),
  ]);

  let f = |t| {
    Note::total_val(notes.iter(), t)
  };

  println!("Writing PCM file!");
  let mut file = File::create("./samples.pcm").unwrap();
  play(&mut file, &f, 5.0);

  println!("Searching for freqs!");

  let mut t = 0.125;
  while t < 5.0 {
    for detected_pitch in ScaleScanner::new(&f, t) {
      println!("{}: {:?}", t, detected_pitch);
    }

    t += 0.25;
  }
}
