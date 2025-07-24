# Program
Status: 🟢
Assertions: 0

```rustleaf
loop {
    break;
}
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
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
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
                                None,
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
                            None,
                        ),
                    ],
                    None,
                ),
            ),
        ),
    ),
)
```