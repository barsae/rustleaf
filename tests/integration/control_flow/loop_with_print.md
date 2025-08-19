# Program
Status: 🟢
Assertions: 1

```rustleaf
assert((loop {
    break 42;
}) == 42);
```

# Output
None

# Result
```rust
Ok(
    Unit,
)
```

# Lex
```rust
Ok(
    [
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(LeftParen),
        3: Token(Loop),
        4: Token(LeftBrace),
        5: Token(Break),
        6: Token(Int, "42"),
        7: Token(Semicolon),
        8: Token(RightBrace),
        9: Token(RightParen),
        10: Token(EqualEqual),
        11: Token(Int, "42"),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Eof)
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
                        "assert",
                    ),
                    [
                        Eq(
                            Loop {
                                body: Block {
                                    statements: [
                                        Break(
                                            Some(
                                                Literal(
                                                    Int(
                                                        42,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                    final_expr: None,
                                },
                            },
                            Literal(
                                Int(
                                    42,
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
    RustValue(<unknown>),
)
```