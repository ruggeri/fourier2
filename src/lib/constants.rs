pub const FOURIER_SAMPLE_RATE_PER_SEC: f64 = SAMPLE_RATE / 1.0_f64;
// It's hard to detect lower notes accurately without a greater window.
pub const FOURIER_WINDOW_WIDTH: f64 = 0.05_f64;
pub const NOISE_LEVEL: f64 = 0.1_f64;
pub const SAMPLE_RATE: f64 = 44_100.0;
pub const SCAN_HZ_RESOLUTION: f64 = 1.00_f64;
pub const SCAN_AMPLITUDE_THRESHOLD: f64 = 0.001_f64;
pub const SCAN_TIME_RESOLUTION: f64 = 0.1_f64;
pub const SCAN_SMOOTHING_RATIO: f64 = 0.75_f64;