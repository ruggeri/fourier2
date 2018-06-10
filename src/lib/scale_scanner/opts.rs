use constants::*;
use transforms::{FourierTransformOpts, FourierTransformOptsBuilder};

#[derive(Builder, Clone, Copy)]
pub struct ScaleScannerOpts {
    // Minimum amplitude required for a frequency to be reportred as detected.
    #[builder(default = "DEFAULT_SCAN_AMPLITUDE_MIN_THRESHOLD")]
    pub scan_amplitude_min_threshold: f64,
    // How often do we perform a scan?
    #[builder(default = "DEFAULT_SCAN_TIME_RESOLUTION")]
    pub scan_time_resolution: f64,

    #[builder(default = "DEFAULT_FOURIER_WINDOW_WIDTH")]
    pub fourier_window_width: f64,
    #[builder(default = "DEFAULT_FOURIER_SAMPLE_RATE")]
    pub fourier_sample_rate: f64,
}

impl From<ScaleScannerOpts> for FourierTransformOpts {
    fn from(opts: ScaleScannerOpts) -> FourierTransformOpts {
        FourierTransformOptsBuilder::default()
            .window_width(opts.fourier_window_width)
            .sample_rate(opts.fourier_sample_rate)
            .build()
            .unwrap()
    }
}
