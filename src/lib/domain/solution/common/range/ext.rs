//! Set operations on half-open [`Range`]s: split, intersect, subtract, merge.

use std::ops::Range;

/// Set operations on a half-open [`Range`]. Every range they give back is
/// non-empty.
pub trait RangeExt<T> {
    /// `self` cut at whichever of `other`'s ends fall inside it, in order.
    fn split(self, other: Range<T>) -> impl Iterator<Item = Range<T>>;
    /// The values in both, or `None` when they share none. Ranges that only
    /// touch share none.
    fn intersection(self, other: Range<T>) -> Option<Range<T>>;
    /// The parts of `self` outside `other`, in order: none, one, or two when
    /// `other` sits inside `self`.
    fn difference(self, other: Range<T>) -> impl Iterator<Item = Range<T>>;
    /// The union: one range when the two overlap or touch, otherwise `self`
    /// then `other` unchanged.
    fn merge(self, other: Range<T>) -> impl Iterator<Item = Range<T>>;
}

impl<T: PartialOrd + Copy> RangeExt<T> for Range<T> {
    fn split(self, other: Range<T>) -> impl Iterator<Item = Range<T>> {
        match (self.contains(&other.start), self.contains(&other.end)) {
            (true, true) => [
                Some(self.start..other.start),
                Some(other.start..other.end),
                Some(other.end..self.end),
            ],
            (true, false) => [
                Some(self.start..other.start),
                Some(other.start..self.end),
                None,
            ],
            (false, true) => [Some(self.start..other.end), Some(other.end..self.end), None],
            (false, false) => [Some(self.start..self.end), None, None],
        }
        .into_iter()
        .flatten()
        .filter(|r| !r.is_empty())
    }

    fn intersection(self, other: Range<T>) -> Option<Range<T>> {
        match (other.contains(&self.start), other.contains(&self.end)) {
            (true, true) => Some(self.start..self.end),
            (true, false) => Some(self.start..other.end),
            (false, true) => Some(other.start..self.end),
            (false, false) if self.contains(&other.start) => Some(other),
            (false, false) => None,
        }
        .filter(|r| !r.is_empty())
    }

    fn difference(self, other: Range<T>) -> impl Iterator<Item = Range<T>> {
        match (other.contains(&self.start), other.contains(&self.end)) {
            (true, true) => [None, None],
            (true, false) => [Some(other.end..self.end), None],
            (false, true) => [Some(self.start..other.start), None],
            (false, false) if self.contains(&other.start) => {
                [Some(self.start..other.start), Some(other.end..self.end)]
            }
            (false, false) => [Some(self.start..self.end), None],
        }
        .into_iter()
        .flatten()
        .filter(|r| !r.is_empty())
    }

    fn merge(self, other: Range<T>) -> impl Iterator<Item = Range<T>> {
        let reach = other.start..=other.end;
        match (reach.contains(&self.start), reach.contains(&self.end)) {
            (true, true) => [Some(other.start..other.end), None],
            (true, false) => [Some(other.start..self.end), None],
            (false, true) => [Some(self.start..other.end), None],
            (false, false) if self.contains(&other.start) => [Some(self.start..self.end), None],
            (false, false) => [Some(self.start..self.end), Some(other)],
        }
        .into_iter()
        .flatten()
        .filter(|r| !r.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_contains() {
        let a = 0..10;
        let b = 6..8;
        assert_eq!(a.split(b).collect::<Vec<_>>(), vec![0..6, 6..8, 8..10]);
    }

    #[test]
    fn split_straddles_start() {
        let a = 4..10;
        let b = 0..7;
        assert_eq!(a.split(b).collect::<Vec<_>>(), vec![4..7, 7..10]);
    }

    #[test]
    fn split_straddles_end() {
        let a = 0..6;
        let b = 3..10;
        assert_eq!(a.split(b).collect::<Vec<_>>(), vec![0..3, 3..6]);
    }

    #[test]
    fn split_disjoint() {
        let a = 0..3;
        let b = 6..10;
        assert_eq!(a.split(b).collect::<Vec<_>>(), vec![0..3]);
    }

    #[test]
    fn intersection_contains() {
        let a = 0..10;
        let b = 4..6;
        assert_eq!(a.intersection(b), Some(4..6));
    }

    #[test]
    fn intersection_within() {
        let a = 4..8;
        let b = 0..10;
        assert_eq!(a.intersection(b), Some(4..8));
    }

    #[test]
    fn intersection_straddles_start() {
        let a = 4..9;
        let b = 1..7;
        assert_eq!(a.intersection(b), Some(4..7));
    }

    #[test]
    fn intersection_straddles_end() {
        let a = 0..5;
        let b = 3..10;
        assert_eq!(a.intersection(b), Some(3..5));
    }

    #[test]
    fn intersection_disjoint() {
        let a = 0..1;
        let b = 3..9;
        assert!(a.intersection(b).is_none());
    }

    #[test]
    fn intersection_touching() {
        let a = 0..5;
        let b = 5..10;
        assert!(a.intersection(b).is_none());
    }

    #[test]
    fn split_shared_start() {
        let a = 0..10;
        let b = 0..5;
        assert_eq!(a.split(b).collect::<Vec<_>>(), vec![0..5, 5..10]);
    }

    #[test]
    fn difference_contains() {
        let a = 0..10;
        let b = 4..6;
        assert_eq!(a.difference(b).collect::<Vec<_>>(), vec![0..4, 6..10]);
    }

    #[test]
    fn difference_within() {
        let a = 4..8;
        let b = 0..10;
        assert_eq!(a.difference(b).next(), None);
    }

    #[test]
    fn difference_straddles_start() {
        let a = 4..9;
        let b = 1..7;
        assert_eq!(a.difference(b).collect::<Vec<_>>(), vec![7..9]);
    }

    #[test]
    fn difference_straddles_end() {
        let a = 0..5;
        let b = 3..10;
        assert_eq!(a.difference(b).collect::<Vec<_>>(), vec![0..3]);
    }

    #[test]
    fn difference_disjoint() {
        let a = 0..1;
        let b = 3..9;
        assert_eq!(a.difference(b).collect::<Vec<_>>(), vec![0..1]);
    }

    #[test]
    fn difference_shared_end() {
        let a = 4..10;
        let b = 0..10;
        assert_eq!(a.difference(b).next(), None);
    }

    #[test]
    fn difference_shared_start() {
        let a = 0..10;
        let b = 0..5;
        assert_eq!(a.difference(b).collect::<Vec<_>>(), vec![5..10]);
    }

    #[test]
    fn merge_contains() {
        let a = 0..10;
        let b = 4..6;
        assert_eq!(a.merge(b).collect::<Vec<_>>(), vec![0..10]);
    }

    #[test]
    fn merge_within() {
        let a = 4..8;
        let b = 0..10;
        assert_eq!(a.merge(b).collect::<Vec<_>>(), vec![0..10]);
    }

    #[test]
    fn merge_straddles_start() {
        let a = 4..9;
        let b = 1..7;
        assert_eq!(a.merge(b).collect::<Vec<_>>(), vec![1..9]);
    }

    #[test]
    fn merge_straddles_end() {
        let a = 0..5;
        let b = 3..10;
        assert_eq!(a.merge(b).collect::<Vec<_>>(), vec![0..10]);
    }

    #[test]
    fn merge_disjoint() {
        let a = 0..1;
        let b = 3..9;
        assert_eq!(a.merge(b).collect::<Vec<_>>(), vec![0..1, 3..9]);
    }

    #[test]
    fn merge_touching() {
        assert_eq!((0..5).merge(5..10).collect::<Vec<_>>(), vec![0..10]);
        assert_eq!((5..10).merge(0..5).collect::<Vec<_>>(), vec![0..10]);
    }
}
