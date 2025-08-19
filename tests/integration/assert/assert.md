# Program
Status: 🟢
Assertions: 3

```rustleaf
assert(true);
assert(1 == 1);
assert(10 + 5 == 15, "Math should work");
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
        2: Token(True),
        3: Token(RightParen),
        4: Token(Semicolon),
        5: Token(Ident, "assert"),
        6: Token(LeftParen),
        7: Token(Int, "1"),
        8: Token(EqualEqual),
        9: Token(Int, "1"),
        10: Token(RightParen),
        11: Token(Semicolon),
        12: Token(Ident, "assert"),
        13: Token(LeftParen),
        14: Token(Int, "10"),
        15: Token(Plus),
        16: Token(Int, "5"),
        17: Token(EqualEqual),
        18: Token(Int, "15"),
        19: Token(Comma),
        20: Token(String, "Math should work"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Eof)
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
                                true,
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Add(
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Math should work",
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