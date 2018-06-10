use fourier2::{
  core::*,
  scale_scanner::*,
  transforms::SmoothablePitchIterator,
  util::PCMFile,
};

pub fn scan_notes<'a>(file: &'a PCMFile, do_smooth: bool, scan_smoothing_ratio: f64, scan_opts: ScaleScannerOpts) -> impl Iterator<Item=Note> + 'a {

  println!("Searching for notes!");
  let mut iter: Box<Iterator<Item=DetectedPitch>>;
  iter = Box::new(ScaleScanner::scan(move |t| file.val(t), 0.0, file.duration(), scan_opts));

  if do_smooth {
    iter = Box::new(iter.smooth(scan_smoothing_ratio));
  }

  let iter = iter.map(move |detected_pitch| {
    println!("t={:0.2} | {:?} | amp={:0.4}", detected_pitch.time, detected_pitch.pitch, detected_pitch.amplitude);
    Note::new(
      detected_pitch.pitch,
      detected_pitch.time,
      // TODO: This is probably an argument??
      scan_opts.scan_time_resolution,
      detected_pitch.amplitude
    )
  });

  iter
}
