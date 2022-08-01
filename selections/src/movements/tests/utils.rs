use std::collections::BTreeMap;

use crate::movements::LineLength;

#[derive(Debug)]
pub(crate) struct TestLineLengths {
    line_length: BTreeMap<usize, usize>,
}

impl TestLineLengths {
    pub(crate) fn new() -> Self {
        let mut line_lengths = TestLineLengths {
            line_length: Default::default(),
        };
        line_lengths.set(0, 0);
        line_lengths
    }

    pub(crate) fn set(&mut self, line: usize, length: usize) {
        self.line_length.insert(line, length);
    }
}

impl LineLength for TestLineLengths {
    fn get_len(&self, line: usize) -> Option<usize> {
        self.line_length.get(&line).map(|x| *x)
    }
}
