use fourier2::{hz_scanner::*, util::PCMFile};

pub fn scan_hz<'a>(
    file: &'a PCMFile,
    opts: HzScannerOpts,
) -> impl Iterator<Item = DetectedHz> + 'a {
    println!("Searching for hz!");
    HzScanner::scan(move |t| file.val(t), 0.0, file.duration(), opts).map(|detected_hz| {
        println!(
            "t={:0.2} | {:0.2}hz | amp={:0.4}",
            detected_hz.time, detected_hz.hz, detected_hz.amplitude
        );
        detected_hz
    })
}
