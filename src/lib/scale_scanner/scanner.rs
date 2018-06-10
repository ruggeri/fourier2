use super::opts::ScaleScannerOpts;
use core::{self, AudioSource, Pitch};
use transforms::{ftransform, FourierTransformOpts};
use util;

pub struct ScaleScanner<'a, AS>
where
    AS: AudioSource + 'a,
{
    piano_pitches: <&'a Vec<Pitch> as IntoIterator>::IntoIter,
    source: &'a AS,
    t: f64,
    opts: ScaleScannerOpts,
}

#[derive(Debug)]
pub struct DetectedPitch {
    pub pitch: Pitch,
    pub amplitude: f64,
    pub time: f64,
}

impl<'a, AS> ScaleScanner<'a, AS>
where
    AS: AudioSource,
{
    pub fn new(source: &'a AS, t: f64, opts: ScaleScannerOpts) -> ScaleScanner<'a, AS> {
        ScaleScanner {
            piano_pitches: core::piano_pitches().iter(),
            source,
            t,
            opts,
        }
    }

    pub fn scan(
        f: &'a AS,
        start_t: f64,
        duration: f64,
        opts: ScaleScannerOpts,
    ) -> impl Iterator<Item = DetectedPitch> + 'a {
        let samples = (duration / opts.scan_time_resolution) as usize;
        (0..samples).flat_map(move |idx| {
            let t = start_t + (idx as f64) * opts.scan_time_resolution;
            ScaleScanner::new(f, t, opts)
        })
    }
}

impl<'a, AS> Iterator for ScaleScanner<'a, AS>
where
    AS: AudioSource,
{
    type Item = DetectedPitch;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&pitch) = self.piano_pitches.next() {
            let coeffs = ftransform(
                pitch.hz as f64,
                self.source,
                self.t,
                FourierTransformOpts::from(self.opts),
            );
            let amplitude = util::amplitude(coeffs);
            if amplitude > self.opts.scan_amplitude_min_threshold {
                return Some(DetectedPitch {
                    pitch,
                    amplitude,
                    time: self.t,
                });
            }
        }

        None
    }
}
