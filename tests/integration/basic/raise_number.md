# Program 🔴
```rustleaf
// #[fail_quietly]
raise(42);
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: raise",
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "raise"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
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
                FunctionCall(
                    Identifier(
                        "raise",
                    ),
                    [
                        Literal(
                            Int(
                                42,
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
            Call(
                Variable(
                    "raise",
                ),
                [
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ],
            ),
        ),
    ),
)
```