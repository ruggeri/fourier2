#![feature(label_break_value)]

extern crate byteorder;
#[macro_use(value_t)]
extern crate clap;
extern crate fourier2;
extern crate rand;

mod args;
mod commands;
mod config;
mod constants;
mod main;

// Helpful command to manipulate PCM files.
// sox -traw -r44100 -b16 -c2 -e signed -L samples.pcm samples.wav

fn main() {
    main::main();
}
