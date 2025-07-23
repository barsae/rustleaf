# Program

```rustleaf
[1, 2, 3, "hello", true];
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "1",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "2",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "3",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "hello",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: True,
            text: None,
        },
        Token {
            token_type: RightBracket,
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
                List(
                    [
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        Literal(
                            Int(
                                2,
                            ),
                        ),
                        Literal(
                            Int(
                                3,
                            ),
                        ),
                        Literal(
                            String(
                                "hello",
                            ),
                        ),
                        Literal(
                            Bool(
                                true,
                            ),
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
            List(
                [
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                    Literal(
                        Int(
                            2,
                        ),
                    ),
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                    Literal(
                        String(
                            "hello",
                        ),
                    ),
                    Literal(
                        Bool(
                            true,
                        ),
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
    "eval not implemented for: List([Literal(Int(1)), Literal(Int(2)), Literal(Int(3)), Literal(String(\"hello\")), Literal(Bool(true))])",
)
```
