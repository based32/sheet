use super::LineLength;
use crate::{Position, Selection, SelectionDeltas, SelectionDirection, SelectionStorage};

impl Selection {
    pub(super) fn move_left(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_left(line_lengths, n);
                let to = if !extend {
                    from.clone()
                } else {
                    self.to.clone()
                };
                Selection::new(to, from)
            }
            SelectionDirection::Forward => {
                let to = self.to.move_left(line_lengths, n);
                let from = if !extend {
                    to.clone()
                } else {
                    self.from.clone()
                };
                Selection::new(from, to)
            }
        }
    }

    pub(super) fn move_right(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_right(line_lengths, n);
                let to = if !extend {
                    from.clone()
                } else {
                    self.to.clone()
                };
                Selection::new(to, from)
            }
            SelectionDirection::Forward => {
                let to = self.to.move_right(line_lengths, n);
                let from = if !extend {
                    to.clone()
                } else {
                    self.from.clone()
                };
                Selection::new(from, to)
            }
        }
    }

    pub(super) fn move_up(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_up(line_lengths, n);
                let to = if !extend {
                    from.clone().remove_sticky()
                } else {
                    self.to.clone()
                };
                Selection::new(to, from)
            }
            SelectionDirection::Forward => {
                let to = self.to.move_up(line_lengths, n);
                let from = if !extend {
                    to.clone().remove_sticky()
                } else {
                    self.from.clone()
                };
                Selection::new(from, to)
            }
        }
    }

    pub(super) fn move_down(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_down(line_lengths, n);
                let to = if !extend {
                    from.clone()
                } else {
                    self.to.clone()
                };
                Selection::new(from, to)
            }
            SelectionDirection::Forward => {
                let to = self.to.move_down(line_lengths, n);
                let from = if !extend {
                    to.clone()
                } else {
                    self.from.clone()
                };
                Selection::new(from, to)
            }
        }
    }
}
