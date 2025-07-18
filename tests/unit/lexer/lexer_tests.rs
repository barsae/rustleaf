use rustleaf::{Lexer, LiteralValue, TokenType};

#[test]
fn lexer_keywords() {
    let tokens = Lexer::new("var fn if else while for true false null").unwrap();

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
fn lexer_identifiers() {
    let tokens = Lexer::new("identifier _private camelCase snake_case CONSTANT x123").unwrap();

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
fn lexer_integer_literals() {
    let tokens = Lexer::new("42 1_000_000 0xFF 0xff 0o77 0b1010 0b1111_0000").unwrap();
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
fn lexer_float_literals() {
    let tokens = Lexer::new("3.14159 1.0 0.1 .5 42. 1_234.567_890 1e10 2.5e-4 1E+6").unwrap();
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
fn lexer_float_edge_cases() {
    // Test the specific edge cases mentioned in the review
    let tokens = Lexer::new("42. 42.e10 0. 123.e-5").unwrap();
    assert_eq!(tokens.len(), 5); // 4 floats + EOF

    // All should be float literals
    for i in 0..4 {
        assert_eq!(
            tokens[i].token_type,
            TokenType::FloatLiteral,
            "Token {} should be a float: {:?}",
            i,
            tokens[i]
        );
    }

    // Check values
    assert_eq!(tokens[0].value, Some(LiteralValue::Float(42.0)));
    assert_eq!(tokens[1].value, Some(LiteralValue::Float(42e10)));
    assert_eq!(tokens[2].value, Some(LiteralValue::Float(0.0)));
    assert_eq!(tokens[3].value, Some(LiteralValue::Float(123e-5)));
}

#[test]
fn lexer_string_literals() {
    let tokens = Lexer::new(r#""Hello, world!" "Line 1\nLine 2" "Unicode: \u{1F604}""#).unwrap();
    assert_eq!(tokens.len(), 4); // 3 strings + EOF

    for i in 0..3 {
        assert_eq!(tokens[i].token_type, TokenType::StringLiteral);
    }

    assert_eq!(
        tokens[0].value,
        Some(LiteralValue::String("Hello, world!".to_string()))
    );
    assert_eq!(
        tokens[1].value,
        Some(LiteralValue::String("Line 1\nLine 2".to_string()))
    );
    assert_eq!(
        tokens[2].value,
        Some(LiteralValue::String("Unicode: 😄".to_string()))
    );
}

#[test]
fn lexer_triple_quoted_strings() {
    let tokens = Lexer::new(
        r#""""This is a
multi-line string
with preserved formatting""""#,
    )
    .unwrap();
    assert_eq!(tokens.len(), 2); // 1 string + EOF

    assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
    assert_eq!(
        tokens[0].value,
        Some(LiteralValue::String(
            "This is a\nmulti-line string\nwith preserved formatting".to_string()
        ))
    );
}

#[test]
fn lexer_raw_strings() {
    let tokens = Lexer::new(r#"r"C:\Users\Name\Documents" r"\n is not a newline""#).unwrap();
    assert_eq!(tokens.len(), 3); // 2 raw strings + EOF

    for i in 0..2 {
        assert_eq!(tokens[i].token_type, TokenType::RawStringLiteral);
    }

    assert_eq!(
        tokens[0].value,
        Some(LiteralValue::String(r"C:\Users\Name\Documents".to_string()))
    );
    assert_eq!(
        tokens[1].value,
        Some(LiteralValue::String(r"\n is not a newline".to_string()))
    );
}

#[test]
fn lexer_operators() {
    let tokens = Lexer::new("+ - * / % ** += -= *= /= %= == != < > <= >= & | ^ ~ << >>").unwrap();
    assert_eq!(tokens.len(), 24); // 23 operators + EOF

    let expected_types = vec![
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Star,
        TokenType::Slash,
        TokenType::Percent,
        TokenType::StarStar,
        TokenType::PlusEqual,
        TokenType::MinusEqual,
        TokenType::StarEqual,
        TokenType::SlashEqual,
        TokenType::PercentEqual,
        TokenType::EqualEqual,
        TokenType::BangEqual,
        TokenType::Less,
        TokenType::Greater,
        TokenType::LessEqual,
        TokenType::GreaterEqual,
        TokenType::Ampersand,
        TokenType::Pipe,
        TokenType::Caret,
        TokenType::Tilde,
        TokenType::LessLess,
        TokenType::GreaterGreater,
    ];

    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn lexer_punctuation() {
    let tokens = Lexer::new("( ) { } [ ] , . : :: ; ->").unwrap();

    assert_eq!(tokens.len(), 13); // 12 punctuation + EOF

    let expected_types = vec![
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::LeftBracket,
        TokenType::RightBracket,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::Colon,
        TokenType::DoubleColon,
        TokenType::Semicolon,
        TokenType::Arrow,
    ];

    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn lexer_comments() {
    let tokens = Lexer::new("// This is a single-line comment\n/// Doc comment\n/* Multi-line\n   comment */\n/** Doc block */").unwrap();

    // Find comment tokens
    let comment_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(t.token_type, TokenType::Comment | TokenType::DocComment))
        .collect();

    assert_eq!(comment_tokens.len(), 4);
    assert_eq!(comment_tokens[0].token_type, TokenType::Comment);
    assert_eq!(comment_tokens[1].token_type, TokenType::DocComment);
    assert_eq!(comment_tokens[2].token_type, TokenType::Comment);
    assert_eq!(comment_tokens[3].token_type, TokenType::DocComment);
}

#[test]
fn lexer_newlines() {
    let tokens = Lexer::new("line1\nline2\r\nline3\rline4").unwrap();

    // Should have identifiers but no newline tokens (newlines are treated as whitespace)
    let identifier_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Identifier)
        .collect();
    assert_eq!(identifier_tokens.len(), 4); // line1, line2, line3, line4

    // Verify no newline tokens are present
    let has_newlines = tokens
        .iter()
        .any(|t| matches!(t.lexeme.as_str(), "\n" | "\r" | "\r\n"));
    assert!(!has_newlines);
}

#[test]
fn lexer_mixed_tokens() {
    let tokens = Lexer::new("var count = 42\nfn calculate(x) { x * 2 }").unwrap();

    let expected_types = vec![
        TokenType::Var,
        TokenType::Identifier,
        TokenType::Equal,
        TokenType::IntegerLiteral,
        // No newline token - newlines are treated as whitespace
        TokenType::Fn,
        TokenType::Identifier,
        TokenType::LeftParen,
        TokenType::Identifier,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::Identifier,
        TokenType::Star,
        TokenType::IntegerLiteral,
        TokenType::RightBrace,
        TokenType::Eof,
    ];

    assert_eq!(tokens.len(), expected_types.len());

    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type, "Token {} mismatch", i);
    }
}

#[test]
fn lexer_whitespace_handling() {
    let tokens = Lexer::new("var   x   =   42").unwrap();
    assert_eq!(tokens.len(), 5); // var, x, =, 42, EOF

    // Whitespace should be skipped
    assert_eq!(tokens[0].token_type, TokenType::Var);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Equal);
    assert_eq!(tokens[3].token_type, TokenType::IntegerLiteral);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}

#[test]
fn lexer_position_tracking() {
    let tokens = Lexer::new("var x\n= 42").unwrap();

    // Check line and column positions
    assert_eq!(tokens[0].location.line, 1); // var
    assert_eq!(tokens[0].location.column, 1);

    assert_eq!(tokens[1].location.line, 1); // x
    assert_eq!(tokens[1].location.column, 5);

    // No newline token - newlines are treated as whitespace
    assert_eq!(tokens[2].location.line, 2); // =
    assert_eq!(tokens[2].location.column, 1);

    assert_eq!(tokens[3].location.line, 2); // 42
    assert_eq!(tokens[3].location.column, 3);
}

#[test]
fn lexer_error_handling() {
    let result = Lexer::new_warnings("var x = @invalid");
    assert!(result.is_err(), "Should have errors");
    let err = result.unwrap_err();

    assert_eq!(err.len(), 1);
    assert!(err
        .iter()
        .next()
        .unwrap()
        .message
        .contains("Unexpected character"));
}

#[test]
fn lexer_unterminated_string() {
    let result = Lexer::new_warnings(r#"var s = "unterminated"#);
    assert!(result.is_err(), "Should have errors");
    let err = result.unwrap_err();

    assert!(err
        .iter()
        .next()
        .unwrap()
        .message
        .contains("Unterminated string"));
}

#[test]
fn lexer_invalid_escape_sequence() {
    let result = Lexer::new_warnings(r#""invalid \x escape""#);
    assert!(result.is_err(), "Should have errors");
    let err = result.unwrap_err();

    assert!(err
        .iter()
        .next()
        .unwrap()
        .message
        .contains("Invalid escape sequence"));
}

#[test]
fn lexer_leading_zeros_error() {
    let result = Lexer::new_warnings("012");
    assert!(result.is_err(), "Should have errors");
    let err = result.unwrap_err();

    assert!(err
        .iter()
        .next()
        .unwrap()
        .message
        .contains("Leading zeros not allowed"));
}

#[test]
fn lexer_invalid_hex_literal() {
    let result = Lexer::new_warnings("0x");
    assert!(result.is_err(), "Should have errors");
    let err = result.unwrap_err();

    assert!(err
        .iter()
        .next()
        .unwrap()
        .message
        .contains("Invalid hexadecimal literal"));
}

#[test]
fn lexer_nested_comments() {
    let tokens = Lexer::new("/* outer /* inner */ still in outer */").unwrap();

    let comment_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Comment)
        .collect();
    assert_eq!(comment_tokens.len(), 1);
    assert_eq!(
        comment_tokens[0].lexeme,
        "/* outer /* inner */ still in outer */"
    );
}

#[test]
fn lexer_unicode_in_strings() {
    let tokens = Lexer::new(r#""Hello, 世界! 🌍""#).unwrap();
    assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
    assert_eq!(
        tokens[0].value,
        Some(LiteralValue::String("Hello, 世界! 🌍".to_string()))
    );
}

#[test]
fn lexer_unicode_escape() {
    let tokens = Lexer::new(r#""\u{1F604}""#).unwrap();
    assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
    assert_eq!(
        tokens[0].value,
        Some(LiteralValue::String("😄".to_string()))
    );
}

#[test]
fn lexer_bom_handling() {
    let input_with_bom = format!("{}{}", '\u{FEFF}', "var x = 42");
    let tokens = Lexer::new(&input_with_bom).unwrap();
    assert_eq!(tokens[0].token_type, TokenType::Var);
    assert_eq!(tokens[0].location.column, 1); // BOM should not affect column numbering
}

#[test]
fn lexer_all_keywords_exhaustive() {
    let keywords = "and break case catch class continue else false finally fn for from if in is match not null of or pub require return self static super true try use var while with xor";
    let tokens = Lexer::new(keywords).unwrap();

    // Should have all keywords + EOF
    let keyword_count = keywords.split_whitespace().count();
    assert_eq!(tokens.len(), keyword_count + 1);

    // All tokens except EOF should be keywords (not identifiers)
    for token in &tokens[0..keyword_count] {
        assert_ne!(
            token.token_type,
            TokenType::Identifier,
            "Token '{}' should be a keyword",
            token.lexeme
        );
    }
}

#[test]
fn lexer_file_size_warning() {
    // Create a string > 10MB but < 100MB
    let large_content = "x".repeat(11 * 1024 * 1024); // 11MB
    let result = Lexer::new_warnings(&large_content);
    assert!(result.is_ok(), "Should succeed despite warning");
    let (_, warnings) = result.unwrap();
    assert!(!warnings.is_empty(), "Should have file size warnings");
    assert!(
        warnings[0].message.contains("10 MB"),
        "Warning should mention 10 MB limit"
    );
    assert!(
        warnings[0].message.contains("11.0 MB"),
        "Warning should show actual file size"
    );
}

#[test]
fn lexer_file_size_no_warning() {
    // Create a small string < 10MB
    let small_content = "var x = 42\n".repeat(1000); // Much less than 10MB
    let result = Lexer::new_warnings(&small_content);
    assert!(result.is_ok(), "Should succeed");
    let (_, warnings) = result.unwrap();
    assert!(
        warnings.is_empty(),
        "Should have no warnings for small files"
    );
}

#[test]
fn lexer_file_size_limit_exceeded() {
    // Create a string > 100MB (this should return an Err)
    // Note: This test may be slow and use a lot of memory
    let huge_content = "x".repeat(101 * 1024 * 1024); // 101MB
    let result = Lexer::new_warnings(&huge_content);

    assert!(result.is_err(), "Should return error for files > 100MB");
    let err = result.unwrap_err();
    let error_msg = err.iter().next().unwrap().message.as_str();
    assert!(
        error_msg.contains("101.0 MB"),
        "Error should show actual file size"
    );
    assert!(
        error_msg.contains("100 MB"),
        "Error should mention 100 MB limit"
    );
}

#[test]
fn lexer_macro_start() {
    let tokens = Lexer::new("#[test]").unwrap();

    assert_eq!(tokens.len(), 4); // #[, identifier, ], EOF
    assert_eq!(tokens[0].token_type, TokenType::MacroStart);
    assert_eq!(tokens[0].lexeme, "#[");
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].lexeme, "test");
    assert_eq!(tokens[2].token_type, TokenType::RightBracket);
    assert_eq!(tokens[2].lexeme, "]");
}

#[test]
fn lexer_hash_without_bracket() {
    let tokens = Lexer::new("# comment").unwrap();

    assert_eq!(tokens.len(), 3); // #, identifier, EOF
    assert_eq!(tokens[0].token_type, TokenType::Hash);
    assert_eq!(tokens[0].lexeme, "#");
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].lexeme, "comment");
}

#[test]
fn lexer_macro_with_parameters() {
    let tokens = Lexer::new("#[macro_name(param1, param2)]").unwrap();

    let expected_types = vec![
        TokenType::MacroStart,   // #[
        TokenType::Identifier,   // macro_name
        TokenType::LeftParen,    // (
        TokenType::Identifier,   // param1
        TokenType::Comma,        // ,
        TokenType::Identifier,   // param2
        TokenType::RightParen,   // )
        TokenType::RightBracket, // ]
        TokenType::Eof,
    ];

    assert_eq!(tokens.len(), expected_types.len());

    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type, "Token {} mismatch", i);
    }

    assert_eq!(tokens[0].lexeme, "#[");
    assert_eq!(tokens[1].lexeme, "macro_name");
}

#[test]
fn lexer_macro_with_named_parameters() {
    let tokens = Lexer::new("#[macro_name(key: \"value\", flag: true)]").unwrap();

    let expected_types = vec![
        TokenType::MacroStart,    // #[
        TokenType::Identifier,    // macro_name
        TokenType::LeftParen,     // (
        TokenType::Identifier,    // key
        TokenType::Colon,         // :
        TokenType::StringLiteral, // "value"
        TokenType::Comma,         // ,
        TokenType::Identifier,    // flag
        TokenType::Colon,         // :
        TokenType::True,          // true
        TokenType::RightParen,    // )
        TokenType::RightBracket,  // ]
        TokenType::Eof,
    ];

    assert_eq!(tokens.len(), expected_types.len());

    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type, "Token {} mismatch", i);
    }
}

#[test]
fn lexer_multiple_macros() {
    let tokens =
        Lexer::new("#[first_macro]\n#[second_macro(config: \"production\")]\n#[third_macro]")
            .unwrap();

    // Should tokenize each macro start correctly
    let macro_starts: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::MacroStart)
        .collect();

    assert_eq!(macro_starts.len(), 3);

    // Check positioning
    assert_eq!(macro_starts[0].location.line, 1);
    assert_eq!(macro_starts[0].location.column, 1);
    assert_eq!(macro_starts[1].location.line, 2);
    assert_eq!(macro_starts[1].location.column, 1);
    assert_eq!(macro_starts[2].location.line, 3);
    assert_eq!(macro_starts[2].location.column, 1);
}

#[test]
fn lexer_macro_function_decoration() {
    let tokens = Lexer::new("#[test]\nfn test_function() {\n    assert(2 + 2 == 4)\n}").unwrap();

    // Should start with macro tokens followed by function tokens
    assert_eq!(tokens[0].token_type, TokenType::MacroStart);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].lexeme, "test");
    assert_eq!(tokens[2].token_type, TokenType::RightBracket);
    // No newline token - newlines are treated as whitespace
    assert_eq!(tokens[3].token_type, TokenType::Fn);
    assert_eq!(tokens[4].token_type, TokenType::Identifier);
    assert_eq!(tokens[4].lexeme, "test_function");
}

#[test]
fn lexer_macro_position_tracking() {
    let tokens = Lexer::new("#[benchmark]\nfn calculate() {}").unwrap();

    // Check macro positions
    assert_eq!(tokens[0].token_type, TokenType::MacroStart);
    assert_eq!(tokens[0].location.line, 1);
    assert_eq!(tokens[0].location.column, 1);

    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].location.line, 1);
    assert_eq!(tokens[1].location.column, 3); // After "#["

    assert_eq!(tokens[2].token_type, TokenType::RightBracket);
    assert_eq!(tokens[2].location.line, 1);
    assert_eq!(tokens[2].location.column, 12); // After "benchmark"
}
