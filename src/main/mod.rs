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
mod main;

fn main() {
  main::main();
}
