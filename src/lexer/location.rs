#[derive(Debug, Clone, PartialEq, Default)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, byte_offset: usize) -> Self {
        SourceLocation {
            line,
            column,
            byte_offset,
        }
    }
}
