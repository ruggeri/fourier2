use std::iter::Peekable;
use super::constants::*;
use super::scale_scanner::DetectedPitch;

// TODO: THIS CODE IS AN ABOMINATION.

type VecIter = <Vec<DetectedPitch> as IntoIterator>::IntoIter;

pub struct SmoothedPitchIterator<Iter>
  where Iter: Iterator<Item=DetectedPitch> {
  iter: Peekable<Iter>,
  group_iter: Option<VecIter>,
}

impl<Iter> Iterator for SmoothedPitchIterator<Iter>
  where Iter: Iterator<Item=DetectedPitch> {
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
        match time {
          Some(t) if t != item.time => {
            break;
          },
          _ => time = Some(item.time)
        }
      } else {
        break;
      }

      group.push(self.iter.next().unwrap());
    }

    // println!("{:?}", group);
    if group.len() == 0 {
      return None;
    }

    let max_amplitude = group.iter().fold(0.0_f64, |max, n| max.max(n.amplitude));
    let group = group.into_iter().filter(|n| n.amplitude > (max_amplitude / SCAN_SMOOTHING_RATIO));
    let group = group.collect::<Vec<_>>();
    self.group_iter = Some(group.into_iter());

    self.next()
  }
}

pub trait SmoothablePitchIterator
  where Self: Sized + Iterator<Item=DetectedPitch> {
  fn smooth(self) -> SmoothedPitchIterator<Self>;
}

impl<Iter> SmoothablePitchIterator for Iter
  where Self: Sized + Iterator<Item=DetectedPitch> {

  fn smooth(self) -> SmoothedPitchIterator<Self> {
    SmoothedPitchIterator {
      iter: self.peekable(),
      group_iter: None,
    }
  }
}
