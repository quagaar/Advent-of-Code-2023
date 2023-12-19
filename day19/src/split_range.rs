use std::ops::Range;

pub trait SplitRange<Idx>: Sized {
    fn split(&self, value: Idx) -> (Option<Self>, Option<Self>);
}

impl<Idx: PartialOrd + Copy> SplitRange<Idx> for Range<Idx> {
    fn split(&self, value: Idx) -> (Option<Self>, Option<Self>) {
        if self.start >= value {
            (None, Some(self.clone()))
        } else if self.end <= value {
            (Some(self.clone()), None)
        } else {
            (Some(self.start..value), Some(value..self.end))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_range() {
        assert_eq!((0..10).split(0), (None, Some(0..10)));
        assert_eq!((1..10).split(0), (None, Some(1..10)));
        assert_eq!((0..10).split(1), (Some(0..1), Some(1..10)));
        assert_eq!((0..10).split(5), (Some(0..5), Some(5..10)));
        assert_eq!((0..10).split(9), (Some(0..9), Some(9..10)));
        assert_eq!((0..10).split(10), (Some(0..10), None));
        assert_eq!((0..10).split(11), (Some(0..10), None));
    }
}
