//! Defenition and movement implementation for [Position].

mod movement;
#[cfg(test)]
mod test_movement;

/// Coordinates in a document.
#[derive(Debug, PartialEq, Eq, Ord, Clone)]
pub struct Position {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) sticky_column: Option<usize>,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.column.cmp(&other.column),
            other => other,
        })
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
