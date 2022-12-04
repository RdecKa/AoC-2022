#[derive(Debug)]
pub struct Interval {
    start: i32,
    end: i32,
}

// Input is in format "A-B"
impl From<&str> for Interval {
    fn from(item: &str) -> Self {
        let mut split = item.split("-").into_iter();
        Interval {
            start: (split.next().unwrap().parse().unwrap()),
            end: (split.next().unwrap().parse().unwrap()),
        }
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
