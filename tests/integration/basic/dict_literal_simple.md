# Program

```rustleaf
{"a": 1, "b": 2};
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "a",
            ),
        },
        Token {
            token_type: Colon,
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
            token_type: String,
            text: Some(
                "b",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "2",
            ),
        },
        Token {
            token_type: RightBrace,
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
                Dict(
                    [
                        (
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                        (
                            Literal(
                                String(
                                    "b",
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
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
            Dict(
                [
                    (
                        Literal(
                            String(
                                "a",
                            ),
                        ),
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                    ),
                    (
                        Literal(
                            String(
                                "b",
                            ),
                        ),
                        Literal(
                            Int(
                                2,
                            ),
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
    "eval not implemented for: Dict([(Literal(String(\"a\")), Literal(Int(1))), (Literal(String(\"b\")), Literal(Int(2)))])",
)
```
