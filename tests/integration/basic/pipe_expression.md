# Program

```rustleaf
data : transform() : validate();
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "data",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "transform",
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
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "validate",
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
            token_type: Semicolon,
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
            Expression(
                Pipe(
                    Pipe(
                        Identifier(
                            "data",
                        ),
                        FunctionCall(
                            Identifier(
                                "transform",
                            ),
                            [],
                        ),
                    ),
                    FunctionCall(
                        Identifier(
                            "validate",
                        ),
                        [],
                    ),
                ),
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: Pipe(Pipe(Identifier(\"data\"), FunctionCall(Identifier(\"transform\"), [])), FunctionCall(Identifier(\"validate\"), []))",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Pipe(Pipe(Identifier(\"data\"), FunctionCall(Identifier(\"transform\"), [])), FunctionCall(Identifier(\"validate\"), []))",
)
```
