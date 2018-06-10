use ::constants::*;
use clap::{ArgMatches};
use fourier2::{
  self,
  hz_scanner::{HzScannerOpts, HzScannerOptsBuilder},
  scale_scanner::{ScaleScannerOpts, ScaleScannerOptsBuilder},
};

pub struct Config<'a> {
  pub input_fname: &'a str,
  pub output_fname: &'a str,
  pub do_smooth: bool,
  // TODO: This doesn't belong here!
  pub scan_smoothing_ratio: f64,
}

impl<'a> Config<'a> {
  pub fn new(matches: &'a ArgMatches<'a>) -> Config<'a> {
    let scan_smoothing_ratio = if matches.is_present("smoothing-ratio") {
      value_t!(matches, "smoothing-ratio", f64).unwrap_or_else(|e| e.exit())
    } else {
      fourier2::constants::SCAN_SMOOTHING_RATIO
    };

    Config {
      input_fname: matches.value_of("INPUT").unwrap(),
      output_fname: matches.value_of("OUTPUT").unwrap(),
      do_smooth: matches.is_present("smooth"),
      scan_smoothing_ratio
    }
  }
}

pub fn hz_scanner_opts<'a>(matches: &'a ArgMatches<'a>) -> HzScannerOpts {
  let mut hz_scan_opts_builder = HzScannerOptsBuilder::default();
  hz_scan_opts_builder.start_hz(_MIN_HZ).end_hz(_MAX_HZ);

  if matches.is_present("scan_time_resolution") {
    let scan_time_resolution = value_t!(matches, "scan_time_resolution", f64).unwrap_or_else(|e| e.exit());
    hz_scan_opts_builder.scan_time_resolution(scan_time_resolution);
  }
  hz_scan_opts_builder.build().unwrap()
}

pub fn scale_scanner_opts<'a>(matches: &'a ArgMatches<'a>) -> ScaleScannerOpts {
  let mut scale_scan_opts_builder = ScaleScannerOptsBuilder::default();

  if matches.is_present("scan_time_resolution") {
    let scan_time_resolution = value_t!(matches, "scan_time_resolution", f64).unwrap_or_else(|e| e.exit());
    scale_scan_opts_builder.scan_time_resolution(scan_time_resolution);
  }
  scale_scan_opts_builder.build().unwrap()
}
