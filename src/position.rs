#[derive(Clone)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub(crate) fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub(crate) fn increment(&mut self, c: Option<u8>) {
        match c {
            Some(c) => match c == b'\n' {
                true => {
                    self.line += 1;
                    self.column = 1;
                }
                false => self.column += 1,
            },
            None => {}
        }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
