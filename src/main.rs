#![feature(label_break_value)]

extern crate byteorder;
extern crate fourier2;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use fourier2::{Chord, ftransform, Note, Pitch, scale, util};

fn play(f: &mut File, chord: &Chord, duration: f64) {
  let mut t = 0.0;
  while t < duration {
    let val: f64 = chord.val(t);
    f.write_i16::<LittleEndian>(fourier2::util::f64_to_i16(val)).unwrap();

    t += 1_f64 / fourier2::constants::SAMPLE_RATE;
  }
}

fn main() {
  let pitches: Vec<Pitch> = vec![
    "A.3".parse().unwrap(),
    "C.3".parse().unwrap(),
    "E.3".parse().unwrap(),
    "A.4".parse().unwrap(),
  ];

  let chord = Chord::new(pitches, 0.25, 3.5, 0.1);

  println!("Writing PCM file!");
  let mut file = File::create("./samples.pcm").unwrap();
  play(&mut file, &chord, 4.0);

  println!("Searching for freqs!");
  for p in scale::piano_pitches() {
    let hz = p.freq();
    let coeffs = ftransform(hz as f64, |t| chord.val(t), 2.0);
    let amplitude = util::amplitude(coeffs);
    if amplitude > 0.075 {
      println!("{:?}: {:?}", p, amplitude);
    }
  }
}
