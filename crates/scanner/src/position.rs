pub(crate) struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }

    pub fn increment_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    pub fn increment_column(&mut self) {
        self.column += 1;
    }
}