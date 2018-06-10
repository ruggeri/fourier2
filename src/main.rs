#![feature(label_break_value)]

extern crate byteorder;
#[macro_use(value_t)]
extern crate clap;
extern crate fourier2;
extern crate rand;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

use clap::{Arg, App};
use fourier2::{
  constants::*,
  DetectedHz,
  DetectedPitch,
  HzScanner,
  Note,
  PCMFile,
  ScaleScanner,
  SmoothablePitchIterator,
  SongIterator,
  util
};

const _MIN_HZ: f64 = 200.0_f64;
const _MAX_HZ: f64 = 300.0_f64;

fn _write_output(notes: &Vec<Note>, output_fname: &str) {
  println!("Writing PCM output file!");
  util::play_to_file(output_fname, SongIterator::new(notes));
}

fn _scan_notes<'a>(file: &'a PCMFile, do_smooth: bool, scan_smoothing_ratio: f64) -> impl Iterator<Item=Note> + 'a {

  println!("Searching for notes!");
  let mut iter: Box<Iterator<Item=DetectedPitch>>;
  iter = Box::new(ScaleScanner::scan(move |t| file.val(t), 0.0, file.duration()));

  if do_smooth {
    iter = Box::new(iter.smooth(scan_smoothing_ratio));
  }

  let iter = iter.map(|detected_pitch| {
    println!("t={:0.2} | {:?} | amp={:0.4}", detected_pitch.time, detected_pitch.pitch, detected_pitch.amplitude);
    Note::new(
      detected_pitch.pitch,
      detected_pitch.time,
      SCAN_TIME_RESOLUTION,
      detected_pitch.amplitude
    )
  });

  iter
}

fn _scan_hz<'a>(file: &'a PCMFile) -> impl Iterator<Item=DetectedHz> + 'a {

  println!("Searching for hz!");
  HzScanner::scan(move |t| file.val(t), _MIN_HZ, _MAX_HZ, 0.0, file.duration()).map(|detected_hz| {
    println!("t={:0.2} | {:0.2}hz | amp={:0.4}", detected_hz.time, detected_hz.hz, detected_hz.amplitude);
    detected_hz
  })
}

fn main() {
  let matches = App::new("fourier2")
    .arg(Arg::with_name("smooth")
      .long("smooth")
      .help("should smoothing be used?"))
    .arg(Arg::with_name("smoothing-ratio")
      .long("smoothing-ratio")
      .takes_value(true)
      .requires("smooth")
      .help("notes at less than x% of max note amplitude are dropped"))
    .arg(Arg::with_name("INPUT")
      .help("Input PCM file")
      .required(true)
      .index(1))
    .arg(Arg::with_name("OUTPUT")
      .help("Output PCM file name")
      .required(true)
      .index(2))
    .get_matches();

  let input_fname = &matches.value_of("INPUT").unwrap();
  let output_fname = &matches.value_of("OUTPUT").unwrap();
  let do_smooth = matches.is_present("smooth");
  let scan_smoothing_ratio = if matches.is_present("smoothing-ratio") {
    value_t!(matches.value_of("smoothing-ratio"), f64).unwrap_or_else(|e| {
      e.exit();
    })
  } else {
    SCAN_SMOOTHING_RATIO
  };

  println!("Reading PCM input file!");
  let file = PCMFile::open(input_fname);

  // for _ in _scan_hz(&file) {
  //   ; // Do nothing. Force evaluation.
  // }

  let notes = _scan_notes(&file, do_smooth, scan_smoothing_ratio).collect::<Vec<Note>>();
  _write_output(&notes, output_fname);
}
