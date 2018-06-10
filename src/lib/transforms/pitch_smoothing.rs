#![allow(unknown_lints)]

use constants::*;
use scale_scanner::DetectedPitch;
use std::iter::Peekable;

#[derive(Builder, Clone, Copy)]
pub struct PitchSmoothingOpts {
    // Consider the frequency with the highest amplitude. We're going to
    // drop frequencies with less amplitude. Anything with less than
    // smoothing_percentage * max_freq_amplitude is dropped.
    #[builder(default = "DEFAULT_SMOOTHING_PERCENTAGE")]
    smoothing_percentage: f64,
}

// TODO: THIS CODE IS AN ABOMINATION.

type VecIter = <Vec<DetectedPitch> as IntoIterator>::IntoIter;

pub struct SmoothedPitchIterator<Iter>
where
    Iter: Iterator<Item = DetectedPitch>,
{
    iter: Peekable<Iter>,
    group_iter: Option<VecIter>,
    opts: PitchSmoothingOpts,
}

impl<Iter> Iterator for SmoothedPitchIterator<Iter>
where
    Iter: Iterator<Item = DetectedPitch>,
{
    type Item = DetectedPitch;

    fn next(&mut self) -> Option<DetectedPitch> {
        let mut next_group_val = None;
        {
            if let Some(ref mut group_iter) = &mut self.group_iter {
                next_group_val = group_iter.next();
            }
        }
        if next_group_val.is_none() {
            self.group_iter = None;
        } else {
            return next_group_val;
        }

        let mut time = None;
        let mut group = vec![];
        loop {
            if let Some(item) = self.iter.peek() {
                #[allow(float_cmp)]
                match time {
                    Some(t) if t != item.time => {
                        break;
                    }
                    _ => time = Some(item.time),
                }
            } else {
                break;
            }

            group.push(self.iter.next().unwrap());
        }

        // println!("{:?}", group);
        if group.is_empty() {
            return None;
        }

        let max_amplitude = group.iter().fold(0.0_f64, |max, n| max.max(n.amplitude));
        let smoothing_ratio = self.opts.smoothing_percentage;
        let group = group
            .into_iter()
            .filter(|n| n.amplitude > (max_amplitude * smoothing_ratio));
        let group = group.collect::<Vec<_>>();
        self.group_iter = Some(group.into_iter());

        self.next()
    }
}

pub trait SmoothablePitchIterator
where
    Self: Sized + Iterator<Item = DetectedPitch>,
{
    fn smooth(self, opts: PitchSmoothingOpts) -> SmoothedPitchIterator<Self>;
}

impl<Iter> SmoothablePitchIterator for Iter
where
    Self: Sized + Iterator<Item = DetectedPitch>,
{
    fn smooth(self, opts: PitchSmoothingOpts) -> SmoothedPitchIterator<Self> {
        SmoothedPitchIterator {
            iter: self.peekable(),
            group_iter: None,
            opts,
        }
    }
}
