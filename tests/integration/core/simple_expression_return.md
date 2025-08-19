# Program
Status: 🟡
Assertions: 0

```rustleaf
42
```

# Output
None

# Result
```rust
Ok(
    Int(42),
)
```

# Lex
```rust
Ok(
    [
        0: Token(Int, "42"),
        1: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Expression(
                Literal(
                    Int(
                        42,
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
    RustValue(<unknown>),
)
```