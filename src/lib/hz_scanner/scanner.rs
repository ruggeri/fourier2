use super::opts::HzScannerOpts;
use core::AudioSource;
use transforms::{ftransform, FourierTransformOpts};
use util;

pub struct HzScanner<'a, AS>
where
    AS: AudioSource + 'a,
{
    source: &'a AS,
    t: f64,
    hz: f64,
    opts: HzScannerOpts,
}

#[derive(Debug)]
pub struct DetectedHz {
    pub hz: f64,
    pub amplitude: f64,
    pub time: f64,
}

impl<'a, AS> HzScanner<'a, AS>
where
    AS: AudioSource,
{
    pub fn new(source: &'a AS, t: f64, opts: HzScannerOpts) -> HzScanner<'a, AS> {
        HzScanner {
            source,
            t,
            hz: opts.start_hz,
            opts,
        }
    }

    pub fn scan(
        source: &'a AS,
        start_t: f64,
        duration: f64,
        opts: HzScannerOpts,
    ) -> impl Iterator<Item = DetectedHz> + 'a {
        let samples = (duration / opts.scan_time_resolution) as usize;
        (0..samples).flat_map(move |idx| {
            let t = start_t + (idx as f64) * opts.scan_time_resolution;
            HzScanner::new(source, t, opts)
        })
    }
}

impl<'a, AS> Iterator for HzScanner<'a, AS>
where
    AS: AudioSource,
{
    type Item = DetectedHz;

    fn next(&mut self) -> Option<Self::Item> {
        while self.hz <= self.opts.end_hz {
            let hz = self.hz;
            self.hz += self.opts.scan_hz_resolution;

            let coeffs = ftransform(
                hz,
                self.source,
                self.t,
                FourierTransformOpts::from(self.opts),
            );
            let amplitude = util::amplitude(coeffs);
            if amplitude > self.opts.scan_amplitude_min_threshold {
                return Some(DetectedHz {
                    hz,
                    amplitude,
                    time: self.t,
                });
            }
        }

        None
    }
}
