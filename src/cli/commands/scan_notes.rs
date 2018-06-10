use fourier2::{
    core::*,
    scale_scanner::*,
    transforms::{PitchSmoothingOpts, SmoothablePitchIterator},
    util::PCMFile,
};

pub fn scan_notes<'a>(
    pcm_file: &'a PCMFile,
    scan_opts: ScaleScannerOpts,
    smoothing_opts: Option<PitchSmoothingOpts>,
) -> impl Iterator<Item = Note> + 'a {
    println!("Searching for notes!");
    let mut iter: Box<Iterator<Item = DetectedPitch>>;
    iter = Box::new(ScaleScanner::scan(
        pcm_file,
        0.0,
        pcm_file.duration(),
        scan_opts,
    ));

    if let Some(smoothing_opts) = smoothing_opts {
        iter = Box::new(iter.smooth(smoothing_opts));
    }

    iter.map(move |detected_pitch| {
        println!(
            "t={:0.2} | {:?} | amp={:0.4}",
            detected_pitch.time, detected_pitch.pitch, detected_pitch.amplitude
        );
        Note::new(
            detected_pitch.pitch,
            detected_pitch.time,
            // TODO: This is probably an argument??
            scan_opts.scan_time_resolution,
            detected_pitch.amplitude,
        )
    })
}
