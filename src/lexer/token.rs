use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Keywords
    And,
    Break,
    Case,
    Catch,
    Class,
    Continue,
    Else,
    False,
    Finally,
    Fn,
    For,
    From,
    If,
    In,
    Is,
    Match,
    Not,
    Null,
    Of,
    Or,
    Pub,
    Raise,
    Require,
    Return,
    Self_,
    Static,
    Super,
    True,
    Try,
    Use,
    Var,
    While,
    With,

    // Identifiers and literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    RawStringLiteral,
    BooleanLiteral,
    NullLiteral,

    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    StarStar,       // **
    Equal,          // =
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    PercentEqual,   // %=
    EqualEqual,     // ==
    Bang,           // !
    BangEqual,      // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    Ampersand,      // &
    AmpersandAmpersand, // &&
    Pipe,           // |
    PipePipe,       // ||
    Caret,          // ^
    Tilde,          // ~
    LessLess,       // <<
    GreaterGreater, // >>

    // Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    DoubleColon,  // ::
    Semicolon,    // ;
    Arrow,        // ->

    // Special
    Newline,
    Eof,

    // Comments (preserved for tooling)
    Comment,
    DocComment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
    pub value: Option<LiteralValue>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        column: usize,
        byte_offset: usize,
        value: Option<LiteralValue>,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
            byte_offset,
            value,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}({}) at {}:{}",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}
