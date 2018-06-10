use clap::{App, Arg, ArgMatches};

// What lifetimes does App need? I have no idea...
pub fn matches<'a>() -> ArgMatches<'a> {
    App::new("fourier2")
        .arg(
            Arg::with_name("mode")
                .long("mode")
                .takes_value(true)
                .possible_values(&["hz", "scale"])
                .default_value("scale")
                .help("run in hz or scale searching mode"),
        )
        .arg(
            Arg::with_name("fourier-window-width")
                .long("fourier-window-width")
                .takes_value(true)
                .help("how many seconds to compute fourier transfom over"),
        )
        .arg(
            Arg::with_name("scan-amplitude-min-threshold")
                .long("scan-amplitude-min-threshold")
                .takes_value(true)
                .help("minimum amplitude to report a frequency"),
        )
        .arg(
            Arg::with_name("scan-time-resolution")
                .long("scan-time-resolution")
                .takes_value(true)
                .help("how frequently to perform fourier transform"),
        )
        .arg(
            Arg::with_name("smooth")
                .long("smooth")
                .help("should smoothing be used?"),
        )
        .arg(
            Arg::with_name("smoothing-percentage")
                .long("smoothing-percentage")
                .takes_value(true)
                .requires("smooth")
                .help("notes at less than x% of max note amplitude are dropped"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input PCM file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Output PCM file name")
                .required(true)
                .index(2),
        )
        .get_matches()
}
