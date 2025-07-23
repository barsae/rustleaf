# Program 🔴
```rustleaf
// #[fail_quietly]
a is not b;
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: a",
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "a"),
        Token(Is),
        Token(Not),
        Token(Ident, "b"),
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
                Is(
                    Identifier(
                        "a",
                    ),
                    Not(
                        Identifier(
                            "b",
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
            Is(
                Variable(
                    "a",
                ),
                LogicalNot(
                    Variable(
                        "b",
                    ),
                ),
            ),
        ),
    ),
)
```