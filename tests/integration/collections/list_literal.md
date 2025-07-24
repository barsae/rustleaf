# Program
Status: 🟢
Assertions: 0

```rustleaf
[1, 2, 3, "hello", true];
```

# Output
None

# Result
```rust
Ok(
    List(
        ListRef(
            RefCell {
                value: [
                    Int(
                        1,
                    ),
                    Int(
                        2,
                    ),
                    Int(
                        3,
                    ),
                    String(
                        "hello",
                    ),
                    Bool(
                        true,
                    ),
                ],
            },
        ),
    ),
)
```

# Lex
```rust
Ok(
    [
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(String, "hello"),
        Token(Comma),
        Token(True),
        Token(RightBracket),
        Token(Semicolon),
        Token(Eof),
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
        [],
        Some(
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
    ),
)
```