# Program

```rustleaf
{"x": 10, "y": 20,};
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

# Lex

```rust
Ok(
    [
        Token(LeftBrace),
        Token(String, "x"),
        Token(Colon),
        Token(Int, "10"),
        Token(Comma),
        Token(String, "y"),
        Token(Colon),
        Token(Int, "20"),
        Token(Comma),
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
