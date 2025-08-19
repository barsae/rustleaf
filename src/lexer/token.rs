/// A lexical token in RustLeaf source code
#[derive(Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub text: Option<String>,
}

impl Token {
    /// Create a new token with type and optional text
    pub fn new(token_type: TokenType, text: Option<String>) -> Self {
        Self { token_type, text }
    }

    /// Create a simple token without text (for punctuation, keywords, etc.)
    pub fn simple(token_type: TokenType) -> Self {
        Self::new(token_type, None)
    }

    /// Create a token with text (for literals, identifiers, etc.)
    pub fn with_text(token_type: TokenType, text: &str) -> Self {
        Self::new(token_type, Some(text.to_string()))
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.text {
            Some(text) => write!(f, "Token({:?}, {:?})", self.token_type, text),
            None => write!(f, "Token({:?})", self.token_type),
        }
    }
}

/// Types of tokens in RustLeaf
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    Int,
    Float,
    String,             // "regular strings"
    RawString,          // r"raw strings"
    StringPart,         // For interpolation text parts
    InterpolationStart, // ${
    InterpolationEnd,   // }
    True,
    False,
    Null,

    // Documentation Comments
    DocComment,           // /// content
    DocCommentBlock,      // /** content */
    InnerDocComment,      // //! content
    InnerDocCommentBlock, // /*! content */

    // Identifiers and Keywords
    Ident,
    Var,
    Fn,
    If,
    Else,
    While,
    For,
    Loop,
    Return,
    Break,
    Continue,
    Class,
    Static,
    Import,
    As,
    From,
    Match,
    Case,
    Try,
    Catch,
    With,
    And,
    Or,
    Xor,
    Not,
    In,
    Is,
    NotIn, // not in
    IsNot, // is not
    Pub,
    Use,

    // Macro Keywords
    Macro,

    // Macro Tokens
    Hash, // #

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Pipe,
    DotDot,
    DotDotEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Dot,
    Colon,
    DoubleColon,

    Eof,
}
