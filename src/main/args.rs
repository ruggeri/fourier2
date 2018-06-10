use clap::{Arg, ArgMatches, App};

// What lifetimes does App need? I have no idea...
pub fn matches<'a>() -> ArgMatches<'a> {
  App::new("fourier2")
    .arg(Arg::with_name("smooth")
      .long("smooth")
      .help("should smoothing be used?"))
    .arg(Arg::with_name("smoothing-ratio")
      .long("smoothing-ratio")
      .takes_value(true)
      .requires("smooth")
      .help("notes at less than x% of max note amplitude are dropped"))
    .arg(Arg::with_name("scan_time_resolution")
      .long("scan-time-resolution")
      .takes_value(true))
    .arg(Arg::with_name("INPUT")
      .help("Input PCM file")
      .required(true)
      .index(1))
    .arg(Arg::with_name("OUTPUT")
      .help("Output PCM file name")
      .required(true)
      .index(2))
    .get_matches()
}