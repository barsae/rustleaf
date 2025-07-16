use std::fmt;

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical error at {}:{}: {}", self.line, self.column, self.message)
    }
}

#[derive(Debug, Clone)]
pub struct LexWarning {
    pub message: String,
}

impl fmt::Display for LexWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical warning: {}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct LexErrors {
    pub errors: Vec<LexError>,
}

impl LexErrors {
    pub fn new(errors: Vec<LexError>) -> Self {
        Self { errors }
    }
    
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &LexError> {
        self.errors.iter()
    }
}

impl fmt::Display for LexErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.errors.is_empty() {
            write!(f, "No lexical errors")
        } else if self.errors.len() == 1 {
            write!(f, "{}", self.errors[0])
        } else {
            writeln!(f, "{} lexical errors:", self.errors.len())?;
            for (i, error) in self.errors.iter().enumerate() {
                if i == self.errors.len() - 1 {
                    write!(f, "  {}", error)?;
                } else {
                    writeln!(f, "  {}", error)?;
                }
            }
            Ok(())
        }
    }
}

impl std::error::Error for LexErrors {}