use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Interval {
    start: i32,
    end: i32,
}

// Input is in format "A-B"
impl FromStr for Interval {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        Ok(Interval {
            start: split.next().unwrap().parse()?,
            end: split.next().unwrap().parse()?,
        })
    }
}

impl Interval {
    pub fn contains(&self, other: &Interval) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps_with(&self, other: &Interval) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}
