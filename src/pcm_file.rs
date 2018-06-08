use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::prelude::*;
use std::iter;

pub struct PCMFile {
  pub i16s: Vec<i16>,
}

impl PCMFile {
  pub fn open(fname: &str) -> PCMFile {
    let mut file = File::open(fname).unwrap();
    let mut bytes_vec = Vec::<u8>::new();
    file.read_to_end(&mut bytes_vec).unwrap();
    let mut i16s_vec: Vec<i16> = iter::repeat(0_i16).take(bytes_vec.len() / 2).collect();
    LittleEndian::read_i16_into(&bytes_vec, &mut i16s_vec);

    PCMFile { i16s: i16s_vec }
  }
}
