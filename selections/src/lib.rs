//! Selection storage library.

#![deny(missing_docs)]

mod deltas;
mod position;
mod selection;
mod storage;
#[cfg(test)]
mod test_utils;
mod utils;

pub use deltas::{SelectionDelta, SelectionDeltas};
pub use position::Position;
pub use selection::{Selection, SelectionDirection};
pub use storage::SelectionStorage;

/// Source of line lengthes for a buffer.
pub trait LineLength {
    /// Returns a length for a line specified by its index.
    /// If a line contains string `line` the lenght is 4.
    ///
    /// Newline is not included, so `line` line in a middle of a buffer will
    /// have the same length as `line` in the end of the buffer (meaning there
    /// is no newline symbol).
    ///
    /// `None` is returned if requested line is out of buffer's bounds.
    fn get_len(&self, line: usize) -> Option<usize>;

    /// Returns a total number of lines.
    fn lines_count(&self) -> usize;
}
