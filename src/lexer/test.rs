#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token, TokenType};

    #[test]
    fn test_keywords() {
        let source = "var fn if else while for loop return break continue class static self";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::simple(TokenType::Var),
                Token::simple(TokenType::Fn),
                Token::simple(TokenType::If),
                Token::simple(TokenType::Else),
                Token::simple(TokenType::While),
                Token::simple(TokenType::For),
                Token::simple(TokenType::Loop),
                Token::simple(TokenType::Return),
                Token::simple(TokenType::Break),
                Token::simple(TokenType::Continue),
                Token::simple(TokenType::Class),
                Token::simple(TokenType::Static),
                Token::with_text(TokenType::Ident, "self"),
                Token::simple(TokenType::Eof),
            ]
        );
    }

    #[test]
    fn test_additional_keywords() {
        let source = "pub use raise import match try catch macro";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::simple(TokenType::Pub),
                Token::simple(TokenType::Use),
                Token::with_text(TokenType::Ident, "raise"),
                Token::simple(TokenType::Import),
                Token::simple(TokenType::Match),
                Token::simple(TokenType::Try),
                Token::simple(TokenType::Catch),
                Token::simple(TokenType::Macro),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::Ident, "hello"),
                Token::with_text(TokenType::Ident, "world"),
                Token::with_text(TokenType::Ident, "_private"),
                Token::with_text(TokenType::Ident, "camelCase"),
                Token::with_text(TokenType::Ident, "snake_case"),
                Token::with_text(TokenType::Ident, "CONSTANT"),
                Token::with_text(TokenType::Ident, "x123"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::Int, "42"),
                Token::with_text(TokenType::Int, "0"),
                Token::with_text(TokenType::Int, "123"),
                Token::with_text(TokenType::Int, "1_000_000"),
                Token::with_text(TokenType::Int, "0xFF"),
                Token::with_text(TokenType::Int, "0xff"),
                Token::with_text(TokenType::Int, "0o77"),
                Token::with_text(TokenType::Int, "0b1010"),
                Token::with_text(TokenType::Int, "0b1111_0000"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::Float, "3.14159"),
                Token::with_text(TokenType::Float, "1.0"),
                Token::with_text(TokenType::Float, "0.1"),
                Token::with_text(TokenType::Float, "1e10"),
                Token::with_text(TokenType::Float, "2.5e-4"),
                Token::with_text(TokenType::Float, "1E+6"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::String, "hello"),
                Token::with_text(TokenType::String, "world\\n"),
                Token::with_text(TokenType::RawString, "raw\\string"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::MultilineString, "multi\nline"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::String, "regular"),
                Token::with_text(TokenType::RawString, "raw"),
                Token::with_text(TokenType::MultilineString, "multiline\nstring"),
                Token::simple(TokenType::Eof),
            ]
        );
    }

    #[test]
    fn test_booleans_and_null() {
        let source = "true false null";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::simple(TokenType::True),
                Token::simple(TokenType::False),
                Token::simple(TokenType::Null),
                Token::simple(TokenType::Eof),
            ]
        );
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / % ** = += -= *= /= %= == != < > <= >= & | ^ ~ << >> .. ..=";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::simple(TokenType::Plus),
                Token::simple(TokenType::Minus),
                Token::simple(TokenType::Star),
                Token::simple(TokenType::Slash),
                Token::simple(TokenType::Percent),
                Token::simple(TokenType::StarStar),
                Token::simple(TokenType::Equal),
                Token::simple(TokenType::PlusEqual),
                Token::simple(TokenType::MinusEqual),
                Token::simple(TokenType::StarEqual),
                Token::simple(TokenType::SlashEqual),
                Token::simple(TokenType::PercentEqual),
                Token::simple(TokenType::EqualEqual),
                Token::simple(TokenType::BangEqual),
                Token::simple(TokenType::Less),
                Token::simple(TokenType::Greater),
                Token::simple(TokenType::LessEqual),
                Token::simple(TokenType::GreaterEqual),
                Token::simple(TokenType::Ampersand),
                Token::simple(TokenType::Pipe),
                Token::simple(TokenType::Caret),
                Token::simple(TokenType::Tilde),
                Token::simple(TokenType::LessLess),
                Token::simple(TokenType::GreaterGreater),
                Token::simple(TokenType::DotDot),
                Token::simple(TokenType::DotDotEqual),
                Token::simple(TokenType::Eof),
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
                Token::simple(TokenType::LeftParen),
                Token::simple(TokenType::RightParen),
                Token::simple(TokenType::LeftBrace),
                Token::simple(TokenType::RightBrace),
                Token::simple(TokenType::LeftBracket),
                Token::simple(TokenType::RightBracket),
                Token::simple(TokenType::Comma),
                Token::simple(TokenType::Semicolon),
                Token::simple(TokenType::Dot),
                Token::simple(TokenType::Colon),
                Token::simple(TokenType::DoubleColon),
                Token::simple(TokenType::Hash),
                Token::simple(TokenType::Eof),
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
                Token::simple(TokenType::And),
                Token::simple(TokenType::Or),
                Token::simple(TokenType::Xor),
                Token::simple(TokenType::NotIn), // "not in" gets rewritten to NotIn
                Token::simple(TokenType::Is),
                Token::simple(TokenType::Eof),
            ]
        );
    }

    #[test]
    fn test_macro_tokens() {
        let source = "# macro";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::simple(TokenType::Hash),
                Token::simple(TokenType::Macro),
                Token::simple(TokenType::Eof),
            ]
        );
    }

    #[test]
    fn test_macro_annotation_syntax() {
        let source = "#[test]";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::simple(TokenType::Hash),
                Token::simple(TokenType::LeftBracket),
                Token::with_text(TokenType::Ident, "test"),
                Token::simple(TokenType::RightBracket),
                Token::simple(TokenType::Eof),
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
                Token::simple(TokenType::Var),
                Token::with_text(TokenType::Ident, "x"),
                Token::simple(TokenType::Equal),
                Token::with_text(TokenType::Int, "42"),
                Token::simple(TokenType::Semicolon),
                Token::simple(TokenType::Var),
                Token::with_text(TokenType::Ident, "y"),
                Token::simple(TokenType::Equal),
                Token::with_text(TokenType::Int, "24"),
                Token::simple(TokenType::Semicolon),
                Token::simple(TokenType::Eof),
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
                Token::simple(TokenType::Var),
                Token::with_text(TokenType::Ident, "x"),
                Token::simple(TokenType::Equal),
                Token::with_text(TokenType::Int, "42"),
                Token::simple(TokenType::Semicolon),
                Token::simple(TokenType::Eof),
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
                Token::simple(TokenType::Var),
                Token::with_text(TokenType::Ident, "greeting"),
                Token::simple(TokenType::Equal),
                Token::with_text(TokenType::String, "Hello, "),
                Token::simple(TokenType::Plus),
                Token::with_text(TokenType::Ident, "name"),
                Token::simple(TokenType::Plus),
                Token::with_text(TokenType::String, "!"),
                Token::simple(TokenType::Semicolon),
                Token::simple(TokenType::Eof),
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
                Token::simple(TokenType::Fn),
                Token::with_text(TokenType::Ident, "calculate"),
                Token::simple(TokenType::LeftParen),
                Token::with_text(TokenType::Ident, "x"),
                Token::simple(TokenType::Comma),
                Token::with_text(TokenType::Ident, "y"),
                Token::simple(TokenType::RightParen),
                Token::simple(TokenType::LeftBrace),
                Token::simple(TokenType::Return),
                Token::with_text(TokenType::Ident, "x"),
                Token::simple(TokenType::Star),
                Token::with_text(TokenType::Ident, "y"),
                Token::simple(TokenType::Semicolon),
                Token::simple(TokenType::RightBrace),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::Ident, "x"),
                Token::simple(TokenType::GreaterEqual),
                Token::with_text(TokenType::Ident, "y"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::Ident, "variable"),
                Token::with_text(TokenType::Ident, "variance"),
                Token::simple(TokenType::Eof),
            ]
        );
    }

    #[test]
    fn test_empty_input() {
        let source = "";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(tokens, vec![Token::simple(TokenType::Eof)]);
    }

    #[test]
    fn test_only_whitespace() {
        let source = "   \t  \n  \r\n  ";
        let tokens = Lexer::tokenize(source).unwrap();

        assert_eq!(tokens, vec![Token::simple(TokenType::Eof)]);
    }

    #[test]
    fn test_utf8_bom_handling() {
        // Test that BOM is ignored
        let source_with_bom = "\u{FEFF}var x = 42;";
        let tokens = Lexer::tokenize(source_with_bom).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::simple(TokenType::Var),
                Token::with_text(TokenType::Ident, "x"),
                Token::simple(TokenType::Equal),
                Token::with_text(TokenType::Int, "42"),
                Token::simple(TokenType::Semicolon),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::Int, "0"),
                Token::simple(TokenType::DotDot),
                Token::with_text(TokenType::Int, "10"),
                Token::with_text(TokenType::Int, "0"),
                Token::simple(TokenType::DotDotEqual),
                Token::with_text(TokenType::Int, "10"),
                Token::with_text(TokenType::Int, "1"),
                Token::simple(TokenType::DotDot),
                Token::with_text(TokenType::Ident, "n"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::DocComment, " This is a doc comment"),
                Token::with_text(TokenType::InnerDocComment, " This is an inner doc comment"),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::DocCommentBlock, " Block doc comment "),
                Token::with_text(TokenType::InnerDocCommentBlock, " Inner block doc comment "),
                Token::simple(TokenType::Eof),
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
                Token::with_text(TokenType::DocComment, " Doc comment"),
                Token::with_text(TokenType::DocCommentBlock, " Block doc "),
                Token::simple(TokenType::Eof),
            ]
        );
    }
}
