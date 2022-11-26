//! Convenience utilities.

use std::ops::Deref;

/// `Vec` wrapper to ensure it's sorted after each push operation and all
/// elements are unique. Does almost nothing if `push`es are done in proper
/// order.
#[derive(Debug)]
pub(crate) struct UniqueSortedVec<T> {
    vec: Vec<T>,
}

impl<T> Default for UniqueSortedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for UniqueSortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> UniqueSortedVec<T> {
    pub(crate) fn new() -> Self {
        UniqueSortedVec { vec: Vec::new() }
    }

    pub(crate) fn with_capacity(n: usize) -> Self {
        UniqueSortedVec {
            vec: Vec::with_capacity(n),
        }
    }

    /// Pushes new element to [UniqueSortedVec] maintaining its order.
    pub(crate) fn push(&mut self, value: T)
    where
        T: Ord,
    {
        if self.vec.last().map(|last| last < &value).unwrap_or(true) {
            // If order is maintained just put value in the end
            self.vec.push(value);
        } else {
            // If found (`Ok`), then do nothing, otherwise (`Err`) do insertion in
            // appropriate position:
            if let Err(insert_idx) = self.vec.binary_search(&value) {
                self.vec.insert(insert_idx, value);
            }
        }
    }

    /// Builds [UniqueSortedvec] from iterator holding necessary properties.
    pub(crate) fn from_iter(iter: impl Iterator<Item = T>) -> Self
    where
        T: Ord,
    {
        let mut vec: Vec<_> = iter.collect();
        vec.sort_unstable();
        let ok_indexes: Vec<_> = vec.windows(2).map(|w| w[0] != w[1]).collect();
        let mut ok_indexes_iter = ok_indexes.into_iter().chain(std::iter::once(true));
        vec.retain(|_| {
            ok_indexes_iter
                .next()
                .expect("`ok_indexes` length is known")
        });

        UniqueSortedVec { vec }
    }

    pub(crate) fn take(self) -> Vec<T> {
        self.vec
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn order_maintained_and_unique() {
        let mut vec = UniqueSortedVec::new();
        vec.push(5);
        vec.push(6);
        vec.push(7);
        vec.push(1);
        vec.push(3);
        vec.push(7);
        vec.push(4);

        assert_eq!(vec.as_slice(), &[1, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn from_iter() {
        let test = [8, 1, 3, 3, 7, 4, 2, 0, 8, 3, 2, 2, 8];
        let vec = UniqueSortedVec::from_iter(test.into_iter());
        dbg!(&vec);
        assert_eq!(vec.as_slice(), &[0, 1, 2, 3, 4, 7, 8]);
    }
}
