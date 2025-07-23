# Program
Status: 🟢

```rustleaf
assert("hello, world" == "hello, world");
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello, world"),
        Token(EqualEqual),
        Token(String, "hello, world"),
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
                        "assert",
                    ),
                    [
                        Eq(
                            Literal(
                                String(
                                    "hello, world",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello, world",
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
        [],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Literal(
                                String(
                                    "hello, world",
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "hello, world",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```