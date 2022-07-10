use std::collections::BTreeMap;

use crate::movements::LineLength;

#[derive(Debug, Default)]
pub(crate) struct TestLineLengths {
    line_length: BTreeMap<usize, usize>,
}

impl TestLineLengths {
    pub(crate) fn set(&mut self, line: usize, length: usize) {
        self.line_length.insert(line, length);
    }
}

impl LineLength for TestLineLengths {
    fn get_len(&self, line: usize) -> usize {
        *self.line_length.get(&line).unwrap_or(&0)
    }
}
