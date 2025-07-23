# Program

```rustleaf
func(arg1, arg2);
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "func",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "arg1",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "arg2",
            ),
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
                FunctionCall(
                    Identifier(
                        "func",
                    ),
                    [
                        Identifier(
                            "arg1",
                        ),
                        Identifier(
                            "arg2",
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
            Call(
                Variable(
                    "func",
                ),
                [
                    Variable(
                        "arg1",
                    ),
                    Variable(
                        "arg2",
                    ),
                ],
            ),
        ],
        None,
    ),
)
```

# Output

```

```

# Result

```rust
Err(
    "Undefined variable: func",
)
```
