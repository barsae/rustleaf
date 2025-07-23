# Program
Status: 🟢

```rustleaf
if true { 42 }
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
        Token(If),
        Token(True),
        Token(LeftBrace),
        Token(Int, "42"),
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
                If {
                    condition: Literal(
                        Bool(
                            true,
                        ),
                    ),
                    then_expr: Block {
                        statements: [],
                        final_expr: Some(
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    },
                    else_expr: None,
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
            If(
                Literal(
                    Bool(
                        true,
                    ),
                ),
                Block(
                    [],
                    Some(
                        Literal(
                            Int(
                                42,
                            ),
                        ),
                    ),
                ),
                None,
            ),
        ),
    ),
)
```