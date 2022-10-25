use super::Position;
use crate::LineLength;

impl Position {
    pub(crate) fn move_left(&self, line_lengths: &impl LineLength, mut n: usize) -> Position {
        let mut new_pos = self.clone();
        while n > 0 {
            if new_pos.column < n {
                // If there is no space to move left then the position is a beginning of a
                // buffer.
                if new_pos.line == 0 {
                    new_pos.column = 0;
                    break;
                }

                n -= new_pos.column;
                new_pos.line -= 1;
                new_pos.column = line_lengths
                    .get_len(new_pos.line)
                    .expect("position must be on a valid line")
                    + 1;
            } else {
                new_pos.column -= n;
                break;
            }
        }
        new_pos
    }

    pub(crate) fn move_right(&self, line_lengths: &impl LineLength, mut n: usize) -> Position {
        let mut new_pos = self.clone();
        while n > 0 {
            new_pos.column += n;
            let current_line_length = line_lengths
                .get_len(new_pos.line)
                .expect("position must be on a valid line");
            if new_pos.column > current_line_length {
                if line_lengths.get_len(new_pos.line + 1).is_none() {
                    // Reached buffer end
                    new_pos.column = current_line_length; //.saturating_sub(1);
                    break;
                }
                n = new_pos.column - current_line_length - 1;
                new_pos.line += 1;
                new_pos.column = 0;
            } else {
                break;
            }
        }
        new_pos
    }

    pub(crate) fn move_up(&self, line_lengths: &impl LineLength, n: usize) -> Position {
        let mut new_pos = self.clone();
        new_pos.line = new_pos.line.saturating_sub(n);

        let line_length = line_lengths
            .get_len(new_pos.line)
            .expect("lines above positions always exist");
        if line_length >= new_pos.column {
            if let Some(sticky_column) = new_pos.sticky_column {
                new_pos.column = sticky_column;
            }
            new_pos.sticky_column = None;
        } else {
            new_pos.sticky_column = Some(new_pos.column);
            new_pos.column = line_length;
        }

        new_pos
    }

    pub(crate) fn move_down(&self, line_lengths: &impl LineLength, n: usize) -> Position {
        let mut new_pos = self.clone();
        new_pos.line += n;

        let lines_count = line_lengths.lines_count();
        if new_pos.line >= lines_count {
            new_pos.line = lines_count - 1;
        }

        let line_length = line_lengths
            .get_len(new_pos.line)
            .expect("lines count checked above");
        if line_length >= new_pos.column {
            if let Some(sticky_column) = new_pos.sticky_column {
                new_pos.column = sticky_column;
            }
            new_pos.sticky_column = None;
        } else {
            new_pos.sticky_column = Some(new_pos.column);
            new_pos.column = line_length;
        }

        new_pos
    }
}
