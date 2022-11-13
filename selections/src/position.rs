//! Defenition and movement implementation for [Position].

mod movement;
#[cfg(test)]
mod test_movement;

use std::cmp;

/// Coordinates in a document.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) sticky_column: Option<usize>,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(match self.line.cmp(&other.line) {
            cmp::Ordering::Equal => self.column.cmp(&other.column),
            other => other,
        })
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(&other).expect("total ordering is defined")
    }
}

impl Position {
    /// Creates new position with `line` and `column`.
    pub(crate) fn new(line: usize, column: usize) -> Self {
        Position {
            line,
            column,
            sticky_column: None,
        }
    }

    /// Check for equality ignoring sticky column.
    pub(crate) fn weak_eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }

    #[cfg(test)]
    /// Creates new position with `line`, `column` and `sticky_column`.
    pub(crate) fn new_with_sticky(line: usize, column: usize, sticky_column: usize) -> Self {
        Position {
            line,
            column,
            sticky_column: Some(sticky_column),
        }
    }

    /// Removes sticky column
    pub(crate) fn remove_sticky(mut self) -> Self {
        self.sticky_column = None;
        self
    }

    /// Get `line` coordinate.
    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get `column` coordinate.
    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }
}
