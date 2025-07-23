# Program

```rustleaf
{"a": 1, "b": 2};
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

# Lex

```rust
Ok(
    [
        Token(LeftBrace),
        Token(String, "a"),
        Token(Colon),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "b"),
        Token(Colon),
        Token(Int, "2"),
        Token(RightBrace),
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
