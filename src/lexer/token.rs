/// Token types for RustLeaf

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    String(String),          // "regular strings"
    RawString(String),       // r"raw strings"
    MultilineString(String), // """multiline strings"""
    StringPart(String),      // For interpolation
    True,
    False,
    Null,

    // Documentation Comments
    DocComment(String),           // /// content
    DocCommentBlock(String),      // /** content */
    InnerDocComment(String),      // //! content
    InnerDocCommentBlock(String), // /*! content */

    // Identifiers and Keywords
    Ident(String),
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
    Self_,
    Super,
    Import,
    Export,
    As,
    From,
    Match,
    Case,
    Try,
    Catch,
    Finally,
    With,
    And,
    Or,
    Xor,
    Not,
    In,
    Is,
    Pub,
    Raise,
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
    BangEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LessLess,
    GreaterGreater,
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
