use {args, commands, config};

use fourier2::{
    core::*,
    util::{self, PCMFile},
};

fn _write_output(notes: &[Note], output_fname: &str) {
    println!("Writing PCM output file!");
    util::play_to_file(output_fname, SongIterator::new(notes));
}

pub fn main() {
    let arg_matches = args::matches();
    let config = config::Config::new(&arg_matches);

    println!("Reading PCM input file!");
    let file = PCMFile::open(config.input_fname);

    match config.mode {
        config::Mode::Hz => {
            let hzs = commands::scan_hz(&file, config::hz_scanner_opts(&arg_matches));

            for _ in hzs {
              ; // Do nothing. Force evaluation.
            }
        }
        config::Mode::Scale => {
            let notes = commands::scan_notes(
                &file,
                config::scale_scanner_opts(&arg_matches),
                config::scan_smoothing_opts(&arg_matches),
            ).collect::<Vec<Note>>();
            _write_output(&notes, config.output_fname);
        }
    }
}
