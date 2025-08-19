# Program
Status: 🟢
Assertions: 4

```rustleaf
assert(1 + 2 == 3);
assert(5 - 3 == 2);
assert(4 * 3 == 12);
assert(10 / 2 == 5.0);
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
        2: Token(Int, "1"),
        3: Token(Plus),
        4: Token(Int, "2"),
        5: Token(EqualEqual),
        6: Token(Int, "3"),
        7: Token(RightParen),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Int, "5"),
        12: Token(Minus),
        13: Token(Int, "3"),
        14: Token(EqualEqual),
        15: Token(Int, "2"),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Int, "4"),
        21: Token(Star),
        22: Token(Int, "3"),
        23: Token(EqualEqual),
        24: Token(Int, "12"),
        25: Token(RightParen),
        26: Token(Semicolon),
        27: Token(Ident, "assert"),
        28: Token(LeftParen),
        29: Token(Int, "10"),
        30: Token(Slash),
        31: Token(Int, "2"),
        32: Token(EqualEqual),
        33: Token(Float, "5.0"),
        34: Token(RightParen),
        35: Token(Semicolon),
        36: Token(Eof)
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
                            Add(
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
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
                            Sub(
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
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
                            Mul(
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    12,
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
                            Div(
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    5.0,
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
    RustValue(<unknown>),
)
```