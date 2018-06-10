use byteorder::{ByteOrder, LittleEndian};
use constants::*;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use util;

pub struct PCMFile {
    pub i16s: Vec<i16>,
}

impl PCMFile {
    pub fn open(fname: &str) -> PCMFile {
        let mut file = File::open(fname).unwrap();
        let mut bytes_vec = Vec::<u8>::new();
        file.read_to_end(&mut bytes_vec).unwrap();
        let mut i16s_vec_2chan: Vec<i16> = iter::repeat(0_i16).take(bytes_vec.len() / 2).collect();
        LittleEndian::read_i16_into(&bytes_vec, &mut i16s_vec_2chan);

        let mut i16s_vec_1chan = Vec::<i16>::new();
        let mut idx = 0;
        while idx < i16s_vec_2chan.len() {
            let val = (i16s_vec_2chan[idx] / 2) + (i16s_vec_2chan[idx + 1] / 2);
            i16s_vec_1chan.push(val);
            idx += 2;
        }

        PCMFile {
            i16s: i16s_vec_1chan,
        }
    }

    pub fn val(&self, t: f64) -> f64 {
        let idx = (t * SAMPLE_RATE) as usize;
        util::i16_to_f64(self.i16s[idx])
    }

    pub fn duration(&self) -> f64 {
        (self.i16s.len() as f64) / SAMPLE_RATE
    }
}
