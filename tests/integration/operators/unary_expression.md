# Program
Status: 🟢

```rustleaf
-42;
```

# Output
None

# Result
```rust
Ok(
    Int(
        -42,
    ),
)
```

# Lex
```rust
Ok(
    [
        Token(Minus),
        Token(Int, "42"),
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
                Neg(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
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
            Call(
                GetAttr(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                    "op_neg",
                ),
                [],
            ),
        ),
    ),
)
```