#![feature(label_break_value)]

extern crate byteorder;
#[macro_use(value_t)]
extern crate clap;
extern crate fourier2;
extern crate rand;

// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

mod args;
mod commands;
mod config;
mod constants;

use fourier2::{
  core::*,
  util::{self, PCMFile}
};

fn _write_output(notes: &Vec<Note>, output_fname: &str) {
  println!("Writing PCM output file!");
  util::play_to_file(output_fname, SongIterator::new(notes));
}

fn main() {
  let arg_matches = args::matches();
  let config = config::Config::new(&arg_matches);

  println!("Reading PCM input file!");
  let file = PCMFile::open(config.input_fname);

  // for _ in _scan_hz(&file) {
  //   ; // Do nothing. Force evaluation.
  // }

  let notes = commands::scan_notes(
    &file,
    config.do_smooth,
    config.scan_smoothing_ratio,
    config::scale_scanner_opts(&arg_matches),
  ).collect::<Vec<Note>>();
  _write_output(&notes, config.output_fname);
}
