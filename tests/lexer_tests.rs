use rustleaf::{Lexer, TokenType, LiteralValue};

#[test]
fn test_keywords() {
    let mut lexer = Lexer::new("var fn if else while for true false null");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors");
    assert_eq!(tokens.len(), 10); // 9 keywords + EOF
    
    assert_eq!(tokens[0].token_type, TokenType::Var);
    assert_eq!(tokens[1].token_type, TokenType::Fn);
    assert_eq!(tokens[2].token_type, TokenType::If);
    assert_eq!(tokens[3].token_type, TokenType::Else);
    assert_eq!(tokens[4].token_type, TokenType::While);
    assert_eq!(tokens[5].token_type, TokenType::For);
    assert_eq!(tokens[6].token_type, TokenType::True);
    assert_eq!(tokens[7].token_type, TokenType::False);
    assert_eq!(tokens[8].token_type, TokenType::Null);
    assert_eq!(tokens[9].token_type, TokenType::Eof);
}

#[test]
fn test_identifiers() {
    let mut lexer = Lexer::new("identifier _private camelCase snake_case CONSTANT x123");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors");
    assert_eq!(tokens.len(), 7); // 6 identifiers + EOF
    
    for i in 0..6 {
        assert_eq!(tokens[i].token_type, TokenType::Identifier);
    }
    
    assert_eq!(tokens[0].lexeme, "identifier");
    assert_eq!(tokens[1].lexeme, "_private");
    assert_eq!(tokens[2].lexeme, "camelCase");
    assert_eq!(tokens[3].lexeme, "snake_case");
    assert_eq!(tokens[4].lexeme, "CONSTANT");
    assert_eq!(tokens[5].lexeme, "x123");
}

#[test]
fn test_integer_literals() {
    let mut lexer = Lexer::new("42 1_000_000 0xFF 0xff 0o77 0b1010 0b1111_0000");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 8); // 7 integers + EOF
    
    for i in 0..7 {
        assert_eq!(tokens[i].token_type, TokenType::IntegerLiteral);
    }
    
    // Check values
    assert_eq!(tokens[0].value, Some(LiteralValue::Integer(42)));
    assert_eq!(tokens[1].value, Some(LiteralValue::Integer(1_000_000)));
    assert_eq!(tokens[2].value, Some(LiteralValue::Integer(255)));
    assert_eq!(tokens[3].value, Some(LiteralValue::Integer(255)));
    assert_eq!(tokens[4].value, Some(LiteralValue::Integer(63)));
    assert_eq!(tokens[5].value, Some(LiteralValue::Integer(10)));
    assert_eq!(tokens[6].value, Some(LiteralValue::Integer(240)));
}

#[test]
fn test_float_literals() {
    let mut lexer = Lexer::new("3.14159 1.0 0.1 .5 42. 1_234.567_890 1e10 2.5e-4 1E+6");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 10); // 9 floats + EOF
    
    for i in 0..9 {
        assert_eq!(tokens[i].token_type, TokenType::FloatLiteral);
    }
    
    // Check some values
    assert_eq!(tokens[0].value, Some(LiteralValue::Float(3.14159)));
    assert_eq!(tokens[1].value, Some(LiteralValue::Float(1.0)));
    assert_eq!(tokens[2].value, Some(LiteralValue::Float(0.1)));
    assert_eq!(tokens[3].value, Some(LiteralValue::Float(0.5)));
    assert_eq!(tokens[4].value, Some(LiteralValue::Float(42.0)));
}

#[test]
fn test_float_edge_cases() {
    // Test the specific edge cases mentioned in the review
    let mut lexer = Lexer::new("42. 42.e10 0. 123.e-5");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 5); // 4 floats + EOF
    
    // All should be float literals
    for i in 0..4 {
        assert_eq!(tokens[i].token_type, TokenType::FloatLiteral, "Token {} should be a float: {:?}", i, tokens[i]);
    }
    
    // Check values
    assert_eq!(tokens[0].value, Some(LiteralValue::Float(42.0)));
    assert_eq!(tokens[1].value, Some(LiteralValue::Float(42e10)));
    assert_eq!(tokens[2].value, Some(LiteralValue::Float(0.0)));
    assert_eq!(tokens[3].value, Some(LiteralValue::Float(123e-5)));
}

#[test]
fn test_string_literals() {
    let mut lexer = Lexer::new(r#""Hello, world!" "Line 1\nLine 2" "Unicode: \u{1F604}""#);
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 4); // 3 strings + EOF
    
    for i in 0..3 {
        assert_eq!(tokens[i].token_type, TokenType::StringLiteral);
    }
    
    assert_eq!(tokens[0].value, Some(LiteralValue::String("Hello, world!".to_string())));
    assert_eq!(tokens[1].value, Some(LiteralValue::String("Line 1\nLine 2".to_string())));
    assert_eq!(tokens[2].value, Some(LiteralValue::String("Unicode: 😄".to_string())));
}

#[test]
fn test_triple_quoted_strings() {
    let mut lexer = Lexer::new(r#""""This is a
multi-line string
with preserved formatting""""#);
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 2); // 1 string + EOF
    
    assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[0].value, Some(LiteralValue::String("This is a\nmulti-line string\nwith preserved formatting".to_string())));
}

#[test]
fn test_raw_strings() {
    let mut lexer = Lexer::new(r#"r"C:\Users\Name\Documents" r"\n is not a newline""#);
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 3); // 2 raw strings + EOF
    
    for i in 0..2 {
        assert_eq!(tokens[i].token_type, TokenType::RawStringLiteral);
    }
    
    assert_eq!(tokens[0].value, Some(LiteralValue::String(r"C:\Users\Name\Documents".to_string())));
    assert_eq!(tokens[1].value, Some(LiteralValue::String(r"\n is not a newline".to_string())));
}

#[test]
fn test_operators() {
    let mut lexer = Lexer::new("+ - * / % ** += -= *= /= %= == != < > <= >= & | ^ ~ << >>");
    let (tokens, errors) = lexer.tokenize();
    
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 24); // 23 operators + EOF
    
    let expected_types = vec![
        TokenType::Plus, TokenType::Minus, TokenType::Star, TokenType::Slash, TokenType::Percent,
        TokenType::StarStar, TokenType::PlusEqual, TokenType::MinusEqual, TokenType::StarEqual,
        TokenType::SlashEqual, TokenType::PercentEqual, TokenType::EqualEqual, TokenType::BangEqual,
        TokenType::Less, TokenType::Greater, TokenType::LessEqual, TokenType::GreaterEqual,
        TokenType::Ampersand, TokenType::Pipe, TokenType::Caret, TokenType::Tilde,
        TokenType::LessLess, TokenType::GreaterGreater,
    ];
    
    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn test_punctuation() {
    let mut lexer = Lexer::new("( ) { } [ ] , . : :: ; ->");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 13); // 12 punctuation + EOF
    
    let expected_types = vec![
        TokenType::LeftParen, TokenType::RightParen, TokenType::LeftBrace, TokenType::RightBrace,
        TokenType::LeftBracket, TokenType::RightBracket, TokenType::Comma, TokenType::Dot,
        TokenType::Colon, TokenType::DoubleColon, TokenType::Semicolon, TokenType::Arrow,
    ];
    
    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn test_comments() {
    let mut lexer = Lexer::new("// This is a single-line comment\n/// Doc comment\n/* Multi-line\n   comment */\n/** Doc block */");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    
    // Find comment tokens
    let comment_tokens: Vec<_> = tokens.iter().filter(|t| 
        matches!(t.token_type, TokenType::Comment | TokenType::DocComment)
    ).collect();
    
    assert_eq!(comment_tokens.len(), 4);
    assert_eq!(comment_tokens[0].token_type, TokenType::Comment);
    assert_eq!(comment_tokens[1].token_type, TokenType::DocComment);
    assert_eq!(comment_tokens[2].token_type, TokenType::Comment);
    assert_eq!(comment_tokens[3].token_type, TokenType::DocComment);
}

#[test]
fn test_newlines() {
    let mut lexer = Lexer::new("line1\nline2\r\nline3\rline4");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    
    // Should have identifiers and newlines
    let newline_tokens: Vec<_> = tokens.iter().filter(|t| t.token_type == TokenType::Newline).collect();
    assert_eq!(newline_tokens.len(), 3); // \n, \r\n, \r
}

#[test]
fn test_mixed_tokens() {
    let mut lexer = Lexer::new("var count = 42\nfn calculate(x) { x * 2 }");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    
    let expected_types = vec![
        TokenType::Var, TokenType::Identifier, TokenType::Equal, TokenType::IntegerLiteral,
        TokenType::Newline,
        TokenType::Fn, TokenType::Identifier, TokenType::LeftParen, TokenType::Identifier,
        TokenType::RightParen, TokenType::LeftBrace, TokenType::Identifier, TokenType::Star,
        TokenType::IntegerLiteral, TokenType::RightBrace,
        TokenType::Eof,
    ];
    
    assert_eq!(tokens.len(), expected_types.len());
    
    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type, "Token {} mismatch", i);
    }
}

#[test]
fn test_whitespace_handling() {
    let mut lexer = Lexer::new("var   x   =   42");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens.len(), 5); // var, x, =, 42, EOF
    
    // Whitespace should be skipped
    assert_eq!(tokens[0].token_type, TokenType::Var);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Equal);
    assert_eq!(tokens[3].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}

#[test]
fn test_position_tracking() {
    let mut lexer = Lexer::new("var x\n= 42");
    let (tokens, _) = lexer.tokenize();
    
    // Check line and column positions
    assert_eq!(tokens[0].line, 1); // var
    assert_eq!(tokens[0].column, 1);
    
    assert_eq!(tokens[1].line, 1); // x
    assert_eq!(tokens[1].column, 5);
    
    assert_eq!(tokens[2].line, 1); // newline
    assert_eq!(tokens[2].column, 6);
    
    assert_eq!(tokens[3].line, 2); // =
    assert_eq!(tokens[3].column, 1);
    
    assert_eq!(tokens[4].line, 2); // 42
    assert_eq!(tokens[4].column, 3);
}

#[test]
fn test_error_handling() {
    let mut lexer = Lexer::new("var x = @invalid");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(!errors.is_empty(), "Should have errors");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Unexpected character"));
    
    // Should still parse valid tokens
    assert_eq!(tokens[0].token_type, TokenType::Var);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Equal);
    assert_eq!(tokens[3].token_type, TokenType::Identifier); // "invalid"
}

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new(r#"var s = "unterminated"#);
    let (_, errors) = lexer.tokenize();
    
    assert!(!errors.is_empty(), "Should have errors");
    assert!(errors[0].message.contains("Unterminated string"));
}

#[test]
fn test_invalid_escape_sequence() {
    let mut lexer = Lexer::new(r#""invalid \x escape""#);
    let (_, errors) = lexer.tokenize();
    
    assert!(!errors.is_empty(), "Should have errors");
    assert!(errors[0].message.contains("Invalid escape sequence"));
}

#[test]
fn test_leading_zeros_error() {
    let mut lexer = Lexer::new("012");
    let (_, errors) = lexer.tokenize();
    
    assert!(!errors.is_empty(), "Should have errors");
    assert!(errors[0].message.contains("Leading zeros not allowed"));
}

#[test]
fn test_invalid_hex_literal() {
    let mut lexer = Lexer::new("0x");
    let (_, errors) = lexer.tokenize();
    
    assert!(!errors.is_empty(), "Should have errors");
    assert!(errors[0].message.contains("Invalid hexadecimal literal"));
}

#[test]
fn test_nested_comments() {
    let mut lexer = Lexer::new("/* outer /* inner */ still in outer */");
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    
    let comment_tokens: Vec<_> = tokens.iter().filter(|t| t.token_type == TokenType::Comment).collect();
    assert_eq!(comment_tokens.len(), 1);
    assert_eq!(comment_tokens[0].lexeme, "/* outer /* inner */ still in outer */");
}

#[test]
fn test_unicode_in_strings() {
    let mut lexer = Lexer::new(r#""Hello, 世界! 🌍""#);
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[0].value, Some(LiteralValue::String("Hello, 世界! 🌍".to_string())));
}

#[test]
fn test_unicode_escape() {
    let mut lexer = Lexer::new(r#""\u{1F604}""#);
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[0].value, Some(LiteralValue::String("😄".to_string())));
}

#[test]
fn test_bom_handling() {
    let input_with_bom = format!("{}{}", '\u{FEFF}', "var x = 42");
    let mut lexer = Lexer::new(&input_with_bom);
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(tokens[0].token_type, TokenType::Var);
    assert_eq!(tokens[0].column, 1); // BOM should not affect column numbering
}

#[test]
fn test_all_keywords_exhaustive() {
    let keywords = "and break case catch class continue else false finally fn for from if in is match not null of or pub raise require return self static super true try use var while with";
    let mut lexer = Lexer::new(keywords);
    let (tokens, errors) = lexer.tokenize();
    
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    
    // Should have all keywords + EOF
    let keyword_count = keywords.split_whitespace().count();
    assert_eq!(tokens.len(), keyword_count + 1);
    
    // All tokens except EOF should be keywords (not identifiers)
    for token in &tokens[0..keyword_count] {
        assert_ne!(token.token_type, TokenType::Identifier, "Token '{}' should be a keyword", token.lexeme);
    }
}

#[test]
fn test_file_size_warning() {
    // Create a string > 10MB but < 100MB
    let large_content = "x".repeat(11 * 1024 * 1024); // 11MB
    let lexer = Lexer::new(&large_content);
    
    // Check that warnings were generated
    let warnings = lexer.warnings();
    assert!(!warnings.is_empty(), "Should have file size warnings");
    assert!(warnings[0].message.contains("10 MB"), "Warning should mention 10 MB limit");
    assert!(warnings[0].message.contains("11.0 MB"), "Warning should show actual file size");
}

#[test]
fn test_file_size_no_warning() {
    // Create a small string < 10MB
    let small_content = "var x = 42\n".repeat(1000); // Much less than 10MB
    let lexer = Lexer::new(&small_content);
    
    // Check that no warnings were generated
    let warnings = lexer.warnings();
    assert!(warnings.is_empty(), "Should have no warnings for small files");
}

#[test]
fn test_file_size_limit_exceeded() {
    // Create a string > 100MB (this should return an Err)
    // Note: This test may be slow and use a lot of memory
    let huge_content = "x".repeat(101 * 1024 * 1024); // 101MB
    let result = Lexer::try_new(&huge_content);
    
    assert!(result.is_err(), "Should return error for files > 100MB");
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("101.0 MB"), "Error should show actual file size");
    assert!(error_msg.contains("100 MB"), "Error should mention 100 MB limit");
}