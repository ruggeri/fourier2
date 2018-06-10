#![feature(label_break_value)]

extern crate byteorder;
#[macro_use(value_t)]
extern crate clap;
extern crate fourier2;
extern crate rand;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

mod args;

use fourier2::{
  constants::*,
  core::*,
  hz_scanner::*,
  scale_scanner::*,
  transforms::SmoothablePitchIterator,
  util::{self, PCMFile},
};

const _MIN_HZ: f64 = 200.0_f64;
const _MAX_HZ: f64 = 300.0_f64;

fn _write_output(notes: &Vec<Note>, output_fname: &str) {
  println!("Writing PCM output file!");
  util::play_to_file(output_fname, SongIterator::new(notes));
}

fn _scan_notes<'a>(file: &'a PCMFile, do_smooth: bool, scan_smoothing_ratio: f64, scan_opts: ScaleScannerOpts) -> impl Iterator<Item=Note> + 'a {

  println!("Searching for notes!");
  let mut iter: Box<Iterator<Item=DetectedPitch>>;
  iter = Box::new(ScaleScanner::scan(move |t| file.val(t), 0.0, file.duration(), scan_opts));

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

fn _scan_hz<'a>(file: &'a PCMFile, opts: HzScannerOpts) -> impl Iterator<Item=DetectedHz> + 'a {

  println!("Searching for hz!");
  HzScanner::scan(move |t| file.val(t), 0.0, file.duration(), opts).map(|detected_hz| {
    println!("t={:0.2} | {:0.2}hz | amp={:0.4}", detected_hz.time, detected_hz.hz, detected_hz.amplitude);
    detected_hz
  })
}

fn main() {
  let matches = args::matches();

  let input_fname = &matches.value_of("INPUT").unwrap();
  let output_fname = &matches.value_of("OUTPUT").unwrap();
  let do_smooth = matches.is_present("smooth");
  let scan_smoothing_ratio = if matches.is_present("smoothing-ratio") {
    value_t!(matches, "smoothing-ratio", f64).unwrap_or_else(|e| e.exit())
  } else {
    SCAN_SMOOTHING_RATIO
  };

  println!("Reading PCM input file!");
  let file = PCMFile::open(input_fname);

  // for _ in _scan_hz(&file) {
  //   ; // Do nothing. Force evaluation.
  // }

  let mut scale_scan_opts_builder = ScaleScannerOptsBuilder::default();
  let mut hz_scan_opts_builder = HzScannerOptsBuilder::default();
  hz_scan_opts_builder.start_hz(_MIN_HZ).end_hz(_MAX_HZ);

  if matches.is_present("scan_time_resolution") {
    let scan_time_resolution = value_t!(matches, "scan_time_resolution", f64).unwrap_or_else(|e| e.exit());
    scale_scan_opts_builder.scan_time_resolution(scan_time_resolution);
    hz_scan_opts_builder.scan_time_resolution(scan_time_resolution);
  }
  let scale_scan_opts = scale_scan_opts_builder.build().unwrap();
  let hz_scan_opts = hz_scan_opts_builder.build().unwrap();

  let notes = _scan_notes(&file, do_smooth, scan_smoothing_ratio, scale_scan_opts).collect::<Vec<Note>>();
  _write_output(&notes, output_fname);
}
