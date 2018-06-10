use clap::ArgMatches;
use constants::*;
use fourier2::{
    hz_scanner::{HzScannerOpts, HzScannerOptsBuilder},
    scale_scanner::{ScaleScannerOpts, ScaleScannerOptsBuilder},
    transforms::{PitchSmoothingOpts, PitchSmoothingOptsBuilder},
};

pub enum Mode {
    Hz,
    Scale,
}

pub struct Config<'a> {
    pub mode: Mode,
    pub input_fname: &'a str,
    pub output_fname: &'a str,
}

use self::Mode::*;

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches<'a>) -> Config<'a> {
        let mode = match matches.value_of("mode").unwrap() {
            "hz" => Hz,
            "scale" => Scale,
            _ => unreachable!(),
        };

        Config {
            mode,
            input_fname: matches.value_of("INPUT").unwrap(),
            output_fname: matches.value_of("OUTPUT").unwrap(),
        }
    }
}

pub fn hz_scanner_opts<'a>(matches: &'a ArgMatches<'a>) -> HzScannerOpts {
    let mut hz_scan_opts_builder = HzScannerOptsBuilder::default();
    hz_scan_opts_builder.start_hz(_MIN_HZ).end_hz(_MAX_HZ);

    if matches.is_present("scan-amplitude-min-threshold") {
        let scan_amplitude_min_threshold =
            value_t!(matches, "scan-amplitude-min-threshold", f64).unwrap_or_else(|e| e.exit());
        hz_scan_opts_builder.scan_amplitude_min_threshold(scan_amplitude_min_threshold);
    }

    if matches.is_present("scan-time-resolution") {
        let scan_time_resolution =
            value_t!(matches, "scan-time-resolution", f64).unwrap_or_else(|e| e.exit());
        hz_scan_opts_builder.scan_time_resolution(scan_time_resolution);
    }

    if matches.is_present("fourier-window-width") {
        let fourier_window_width =
            value_t!(matches, "fourier-window-width", f64).unwrap_or_else(|e| e.exit());
        hz_scan_opts_builder.fourier_window_width(fourier_window_width);
    }

    hz_scan_opts_builder.build().unwrap()
}

pub fn scale_scanner_opts<'a>(matches: &'a ArgMatches<'a>) -> ScaleScannerOpts {
    let mut scale_scan_opts_builder = ScaleScannerOptsBuilder::default();

    if matches.is_present("scan-amplitude-min-threshold") {
        let scan_amplitude_min_threshold =
            value_t!(matches, "scan-amplitude-min-threshold", f64).unwrap_or_else(|e| e.exit());
        scale_scan_opts_builder.scan_amplitude_min_threshold(scan_amplitude_min_threshold);
    }

    if matches.is_present("scan-time-resolution") {
        let scan_time_resolution =
            value_t!(matches, "scan-time-resolution", f64).unwrap_or_else(|e| e.exit());
        scale_scan_opts_builder.scan_time_resolution(scan_time_resolution);
    }

    if matches.is_present("fourier-window-width") {
        let fourier_window_width =
            value_t!(matches, "fourier-window-width", f64).unwrap_or_else(|e| e.exit());
        scale_scan_opts_builder.fourier_window_width(fourier_window_width);
    }

    scale_scan_opts_builder.build().unwrap()
}

pub fn scan_smoothing_opts<'a>(matches: &'a ArgMatches<'a>) -> Option<PitchSmoothingOpts> {
    if !matches.is_present("smooth") {
        return None;
    }

    let mut pitch_smoothing_opts = PitchSmoothingOptsBuilder::default();

    if matches.is_present("smoothing-percentage") {
        let smoothing_percentage =
            value_t!(matches, "smoothing-percentage", f64).unwrap_or_else(|e| e.exit());
        pitch_smoothing_opts.smoothing_percentage(smoothing_percentage);
    }

    Some(pitch_smoothing_opts.build().unwrap())
}
