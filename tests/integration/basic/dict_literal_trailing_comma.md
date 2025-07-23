# Program

```rustleaf
{"x": 10, "y": 20,};
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
                "x",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "10",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "y",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "20",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
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
                                    "x",
                                ),
                            ),
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ),
                        (
                            Literal(
                                String(
                                    "y",
                                ),
                            ),
                            Literal(
                                Int(
                                    20,
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
                                "x",
                            ),
                        ),
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                    ),
                    (
                        Literal(
                            String(
                                "y",
                            ),
                        ),
                        Literal(
                            Int(
                                20,
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
    "eval not implemented for: Dict([(Literal(String(\"x\")), Literal(Int(10))), (Literal(String(\"y\")), Literal(Int(20)))])",
)
```
