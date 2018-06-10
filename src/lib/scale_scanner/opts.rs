use constants::*;
use transform::{FourierTransformOpts, FourierTransformOptsBuilder};

#[derive(Builder, Clone, Copy)]
pub struct ScaleScannerOpts {
  #[builder(default = "SCAN_TIME_RESOLUTION")]
  pub scan_time_resolution: f64,
  #[builder(default = "FOURIER_WINDOW_WIDTH")]
  pub window_width: f64,
  #[builder(default = "FOURIER_SAMPLE_RATE_PER_SEC")]
  pub sample_rate_per_sec: f64,
}

impl From<ScaleScannerOpts> for FourierTransformOpts {
  fn from(opts: ScaleScannerOpts) -> FourierTransformOpts {
    FourierTransformOptsBuilder
      ::default()
      .window_width(opts.window_width)
      .sample_rate_per_sec(opts.sample_rate_per_sec)
      .build().unwrap()
  }
}
