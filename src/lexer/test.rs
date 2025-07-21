#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn test_keywords() {
        let source = "var fn if else while for loop return break continue class static self super";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Fn,
                Token::If,
                Token::Else,
                Token::While,
                Token::For,
                Token::Loop,
                Token::Return,
                Token::Break,
                Token::Continue,
                Token::Class,
                Token::Static,
                Token::Self_,
                Token::Super,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_additional_keywords() {
        let source = "pub use raise import export match try catch macro";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Pub,
                Token::Use,
                Token::Raise,
                Token::Import,
                Token::Export,
                Token::Match,
                Token::Try,
                Token::Catch,
                Token::Macro,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let source = "hello world _private camelCase snake_case CONSTANT x123";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("hello".to_string()),
                Token::Ident("world".to_string()),
                Token::Ident("_private".to_string()),
                Token::Ident("camelCase".to_string()),
                Token::Ident("snake_case".to_string()),
                Token::Ident("CONSTANT".to_string()),
                Token::Ident("x123".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_integers() {
        let source = "42 0 123 1_000_000 0xFF 0xff 0o77 0b1010 0b1111_0000";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Int(42),
                Token::Int(0),
                Token::Int(123),
                Token::Int(1000000),
                Token::Int(255),
                Token::Int(255),
                Token::Int(63),
                Token::Int(10),
                Token::Int(240),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_floats() {
        let source = "3.14159 1.0 0.1 1e10 2.5e-4 1E+6";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Float(3.14159),
                Token::Float(1.0),
                Token::Float(0.1),
                Token::Float(1e10),
                Token::Float(2.5e-4),
                Token::Float(1e6),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_strings() {
        let source = r#""hello" "world\n" r"raw\string""#;
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::String("hello".to_string()),
                Token::String("world\\n".to_string()),
                Token::RawString("raw\\string".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_multiline_strings() {
        let source = r#""""multi
line""""#;
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::MultilineString("multi\nline".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_all_string_types() {
        let source = r#""regular" r"raw" """multiline
string""""#;
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::String("regular".to_string()),
                Token::RawString("raw".to_string()),
                Token::MultilineString("multiline\nstring".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_booleans_and_null() {
        let source = "true false null";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![Token::True, Token::False, Token::Null, Token::Eof,]
        );
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / % ** = += -= *= /= %= == != < > <= >= & | ^ ~ << >> .. ..=";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Percent,
                Token::StarStar,
                Token::Equal,
                Token::PlusEqual,
                Token::MinusEqual,
                Token::StarEqual,
                Token::SlashEqual,
                Token::PercentEqual,
                Token::EqualEqual,
                Token::BangEqual,
                Token::Less,
                Token::Greater,
                Token::LessEqual,
                Token::GreaterEqual,
                Token::Ampersand,
                Token::Pipe,
                Token::Caret,
                Token::Tilde,
                Token::LessLess,
                Token::GreaterGreater,
                Token::DotDot,
                Token::DotDotEqual,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_punctuation() {
        let source = "( ) { } [ ] , ; . : :: #";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                Token::RightBrace,
                Token::LeftBracket,
                Token::RightBracket,
                Token::Comma,
                Token::Semicolon,
                Token::Dot,
                Token::Colon,
                Token::DoubleColon,
                Token::Hash,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_logical_keywords() {
        let source = "and or xor not in is";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::And,
                Token::Or,
                Token::Xor,
                Token::Not,
                Token::In,
                Token::Is,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_macro_tokens() {
        let source = "# macro";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(tokens, vec![Token::Hash, Token::Macro, Token::Eof,]);
    }

    #[test]
    fn test_macro_annotation_syntax() {
        let source = "#[test]";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Hash,
                Token::LeftBracket,
                Token::Ident("test".to_string()),
                Token::RightBracket,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_comments_ignored() {
        let source = "var x = 42; // this is a comment\n/* multi\nline */ var y = 24;";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Ident("x".to_string()),
                Token::Equal,
                Token::Int(42),
                Token::Semicolon,
                Token::Var,
                Token::Ident("y".to_string()),
                Token::Equal,
                Token::Int(24),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_whitespace_ignored() {
        let source = "  var   x   =   42  ;  ";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Ident("x".to_string()),
                Token::Equal,
                Token::Int(42),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_complex_expression() {
        let source = r#"var greeting = "Hello, " + name + "!";"#;
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Ident("greeting".to_string()),
                Token::Equal,
                Token::String("Hello, ".to_string()),
                Token::Plus,
                Token::Ident("name".to_string()),
                Token::Plus,
                Token::String("!".to_string()),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_function_definition() {
        let source = "fn calculate(x, y) { return x * y; }";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Ident("calculate".to_string()),
                Token::LeftParen,
                Token::Ident("x".to_string()),
                Token::Comma,
                Token::Ident("y".to_string()),
                Token::RightParen,
                Token::LeftBrace,
                Token::Return,
                Token::Ident("x".to_string()),
                Token::Star,
                Token::Ident("y".to_string()),
                Token::Semicolon,
                Token::RightBrace,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_error_unexpected_character() {
        let source = "var x = $;"; // $ is not a valid token
        let result = Lexer::tokenize(source);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Unexpected character"));
        assert!(error_msg.contains("$"));
    }

    #[test]
    fn test_longest_match_wins() {
        // Test that >= is tokenized as GreaterEqual, not Greater + Equal
        let source = "x >= y";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("x".to_string()),
                Token::GreaterEqual,
                Token::Ident("y".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_keyword_vs_identifier() {
        // Test that "variable" is an identifier, not "var" + "iable"
        let source = "variable variance";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("variable".to_string()),
                Token::Ident("variance".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_empty_input() {
        let source = "";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_only_whitespace() {
        let source = "   \t  \n  \r\n  ";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_utf8_bom_handling() {
        // Test that BOM is ignored
        let source_with_bom = "\u{FEFF}var x = 42;";
        let tokens = Lexer::tokenize(source_with_bom).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Var,
                Token::Ident("x".to_string()),
                Token::Equal,
                Token::Int(42),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_range_operators() {
        let source = "0..10 0..=10 1..n";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Int(0),
                Token::DotDot,
                Token::Int(10),
                Token::Int(0),
                Token::DotDotEqual,
                Token::Int(10),
                Token::Int(1),
                Token::DotDot,
                Token::Ident("n".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_doc_comments() {
        let source = "/// This is a doc comment\n//! This is an inner doc comment";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::DocComment(" This is a doc comment".to_string()),
                Token::InnerDocComment(" This is an inner doc comment".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_doc_comment_blocks() {
        let source = "/** Block doc comment */\n/*! Inner block doc comment */";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::DocCommentBlock(" Block doc comment ".to_string()),
                Token::InnerDocCommentBlock(" Inner block doc comment ".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_doc_comments_vs_regular_comments() {
        let source = "/// Doc comment\n// Regular comment\n/** Block doc */\n/* Regular block */";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::DocComment(" Doc comment".to_string()),
                Token::DocCommentBlock(" Block doc ".to_string()),
                Token::Eof,
            ]
        );
    }
}
