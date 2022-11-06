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

    pub(crate) fn push(&mut self, value: T)
    where
        T: Ord,
    {
        if self.vec.last().map(|last| last < &value).unwrap_or(true) {
            // If order is maintained just put value in the end
            self.vec.push(value);
        } else {
            match self.vec.binary_search(&value) {
                Err(insert_idx) => self.vec.insert(insert_idx, value),
                Ok(_) => {
                    // Already present, do nothing
                }
            }
        }
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
}
