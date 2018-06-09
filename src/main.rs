#![feature(label_break_value)]

extern crate byteorder;
extern crate fourier2;
extern crate rand;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

use fourier2::{
  constants::*,
  DetectedHz,
  HzScanner,
  Note,
  PCMFile,
  ScaleScanner,
  SongIterator,
  util
};

const DURATION: f64 = 10.0_f64;
const MIN_HZ: f64 = 200.0_f64;
const MAX_HZ: f64 = 300.0_f64;

fn _write_output(notes: &Vec<Note>) {
  println!("Writing PCM output file!");
  util::play_to_file("./outputs/samples.pcm", SongIterator::new(notes));
}

fn _scan_notes<F>(f: F) -> impl Iterator<Item=Note>
  where F: Fn(f64) -> f64 + Copy {

  println!("Searching for notes!");
  ScaleScanner::scan(f, 0.0, DURATION).map(|detected_pitch| {
    println!("t={:0.2} | {:?} | amp={:0.4}", detected_pitch.time, detected_pitch.pitch, detected_pitch.amplitude);
    Note::new(
      detected_pitch.pitch,
      detected_pitch.time,
      SCAN_TIME_RESOLUTION,
      detected_pitch.amplitude
    )
  })
}

fn _scan_hz<F>(f: F) -> impl Iterator<Item=DetectedHz>
  where F: Fn(f64) -> f64 + Copy {

  println!("Searching for hz!");
  HzScanner::scan(f, MIN_HZ, MAX_HZ, 0.0, DURATION).map(|detected_hz| {
    println!("t={:0.2} | {:0.2}hz | amp={:0.4}", detected_hz.time, detected_hz.hz, detected_hz.amplitude);
    detected_hz
  })
}

fn main() {
  println!("Reading PCM input file!");
  let file = PCMFile::open("./outputs/rcar.pcm");

  // for _ in _scan_hz(|t| file.val(t)) {
  //   ; // Do nothing. Force evaluation.
  // }

  let notes = _scan_notes(|t| file.val(t)).collect::<Vec<Note>>();
  _write_output(&notes);
}
