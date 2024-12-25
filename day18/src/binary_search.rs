use std::{fmt, ops::Range};

use num_traits::{one, PrimInt};

// Since ranges do not have binary search
pub trait BinarySearchExt<T> {
    fn partition_point<P>(self, predicate: P) -> T
    where
        P: FnMut(&T) -> bool;
}

impl<T: PrimInt + fmt::Debug> BinarySearchExt<T> for Range<T> {
    fn partition_point<P>(self, mut pred: P) -> T
    where
        P: FnMut(&T) -> bool,
    {
        let mut low = self.start;
        let mut high = self.end;

        while low < high {
            let mid = low + ((high - low) >> 1);
            if pred(&mid) {
                low = mid + one();
            } else {
                high = mid;
            }
        }

        low
    }
}
