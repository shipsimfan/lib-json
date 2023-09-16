use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub(super) fn new() -> Self {
        Position { line: 1, column: 1 }
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub(super) fn increment(&mut self, c: char) {
        if c == '\n' {
            self.column = 1;
            self.line += 1;
        } else {
            self.column += 1;
        }
    }
}

impl Into<(usize, usize)> for Position {
    //! Converts the position to (line, column).
    fn into(self) -> (usize, usize) {
        (self.line, self.column)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.line.cmp(&other.line) {
            Ordering::Equal => self.column.cmp(&other.column),
            ordering => ordering,
        })
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
