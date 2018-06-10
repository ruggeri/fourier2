use fourier2::{core::AudioSource, hz_scanner::*};

pub fn scan_hz<'a, AS>(source: &'a AS, opts: HzScannerOpts) -> impl Iterator<Item = DetectedHz> + 'a
where
    AS: AudioSource,
{
    println!("Searching for hz!");
    HzScanner::scan(source, 0.0, source.duration(), opts).map(|detected_hz| {
        println!(
            "t={:0.2} | {:0.2}hz | amp={:0.4}",
            detected_hz.time, detected_hz.hz, detected_hz.amplitude
        );
        detected_hz
    })
}
