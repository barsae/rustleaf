/// Token types for RustLeaf

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    StringPart(String),    // For interpolation
    True,
    False,
    Null,

    // Identifiers and Keywords
    Ident(String),
    Var, Fn, If, Else, While, For, Return, Break, Continue,
    Class, Static, Self_, Import, Export, As, From,
    Match, Case, Try, Catch, Finally, With,
    And, Or, Xor, Not, In, Is,

    // Operators
    Plus, Minus, Star, Slash, Percent, StarStar,
    Equal, PlusEqual, MinusEqual, StarEqual, SlashEqual, PercentEqual,
    EqualEqual, BangEqual, Less, Greater, LessEqual, GreaterEqual,
    Ampersand, Pipe, Caret, Tilde, LessLess, GreaterGreater,

    // Delimiters
    LeftParen, RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
    Comma, Semicolon, Dot, Colon, DoubleColon, Arrow,
    DollarBrace,  // ${ for string interpolation

    Eof,
}