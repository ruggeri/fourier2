#![feature(label_break_value)]

extern crate byteorder;
extern crate fourier2;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use fourier2::{Note, scale};

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
  let notes: Vec<Note> = vec![
    Note::new("A.3".parse().unwrap(), 0.0, 4.0, 0.1),
    Note::new("C.3".parse().unwrap(), 1.0, 4.0, 0.1),
    Note::new("E.3".parse().unwrap(), 2.0, 4.0, 0.1),
    Note::new("A.4".parse().unwrap(), 3.0, 4.0, 0.1),
  ];

  let f = |t| {
    Note::total_val(notes.iter(), t)
  };

  println!("Writing PCM file!");
  let mut file = File::create("./samples.pcm").unwrap();
  play(&mut file, &f, 4.0);

  println!("Searching for freqs!");

  let mut t = 0.5;
  while t < 4.0 {
    println!("{:?}", scale::scan(&f, t));

    t += 1.0;
  }
}
