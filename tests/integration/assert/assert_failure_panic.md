# Program
Status: 🟢
Assertions: 1

```rustleaf
assert(false, "This should fail");
```

# Output
None

# Result
```rust
Err(
    "Assertion failed: This should fail",
)
```

# Lex
```rust
Ok(
    [
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(False),
        3: Token(Comma),
        4: Token(String, "This should fail"),
        5: Token(RightParen),
        6: Token(Semicolon),
        7: Token(Eof)
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
                        Literal(
                            Bool(
                                false,
                            ),
                        ),
                        Literal(
                            String(
                                "This should fail",
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