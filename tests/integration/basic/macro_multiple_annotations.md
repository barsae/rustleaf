# Program

```rustleaf
#[first_macro]
#[second_macro(config: "test")]
#[third_macro]
fn decorated_function() {
    var result = 42;
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Hash,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "first_macro",
            ),
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Hash,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "second_macro",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "config",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "test",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Hash,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "third_macro",
            ),
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Fn,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "decorated_function",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Var,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "result",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "42",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
    ],
)
```

# Parse

```rust
Ok(
    Program(
        [
            FnDecl {
                name: "decorated_function",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "result",
                            ),
                            value: Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        },
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
        ],
    ),
)
```

# Eval

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"decorated_function\", params: [], body: Block { statements: [VarDecl { pattern: Variable(\"result\"), value: Some(Literal(Int(42))) }], final_expr: None }, is_pub: false }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"decorated_function\", params: [], body: Block { statements: [VarDecl { pattern: Variable(\"result\"), value: Some(Literal(Int(42))) }], final_expr: None }, is_pub: false }",
)
```
