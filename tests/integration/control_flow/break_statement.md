# Program
Status: 🟡
Assertions: 0

```rustleaf
loop {
    break 42;
}
```

# Output
None

# Result
```rust
Ok(
    Int(
        42,
    ),
)
```

# Lex
```rust
Ok(
    [
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
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
            Loop(
                Block(
                    [
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
                    None,
                ),
            ),
        ),
    ),
)
```