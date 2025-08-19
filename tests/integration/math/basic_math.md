# Program
Status: 🟢
Assertions: 7

```rustleaf
assert(sqrt(4) == 2.0);
assert(abs(-5) == 5);
assert(floor(3.7) == 3);
assert(ceil(3.2) == 4);
assert(round(3.5) == 4);
assert(min(5, 3) == 3);
assert(max(5, 3) == 5);
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
        2: Token(Ident, "sqrt"),
        3: Token(LeftParen),
        4: Token(Int, "4"),
        5: Token(RightParen),
        6: Token(EqualEqual),
        7: Token(Float, "2.0"),
        8: Token(RightParen),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "abs"),
        13: Token(LeftParen),
        14: Token(Minus),
        15: Token(Int, "5"),
        16: Token(RightParen),
        17: Token(EqualEqual),
        18: Token(Int, "5"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "floor"),
        24: Token(LeftParen),
        25: Token(Float, "3.7"),
        26: Token(RightParen),
        27: Token(EqualEqual),
        28: Token(Int, "3"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Ident, "ceil"),
        34: Token(LeftParen),
        35: Token(Float, "3.2"),
        36: Token(RightParen),
        37: Token(EqualEqual),
        38: Token(Int, "4"),
        39: Token(RightParen),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Ident, "round"),
        44: Token(LeftParen),
        45: Token(Float, "3.5"),
        46: Token(RightParen),
        47: Token(EqualEqual),
        48: Token(Int, "4"),
        49: Token(RightParen),
        50: Token(Semicolon),
        51: Token(Ident, "assert"),
        52: Token(LeftParen),
        53: Token(Ident, "min"),
        54: Token(LeftParen),
        55: Token(Int, "5"),
        56: Token(Comma),
        57: Token(Int, "3"),
        58: Token(RightParen),
        59: Token(EqualEqual),
        60: Token(Int, "3"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Ident, "max"),
        66: Token(LeftParen),
        67: Token(Int, "5"),
        68: Token(Comma),
        69: Token(Int, "3"),
        70: Token(RightParen),
        71: Token(EqualEqual),
        72: Token(Int, "5"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Eof)
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
                            FunctionCall(
                                Identifier(
                                    "sqrt",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Float(
                                    2.0,
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
                            FunctionCall(
                                Identifier(
                                    "abs",
                                ),
                                [
                                    Neg(
                                        Literal(
                                            Int(
                                                5,
                                            ),
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    5,
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
                            FunctionCall(
                                Identifier(
                                    "floor",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.7,
                                        ),
                                    ),
                                ],
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
                            FunctionCall(
                                Identifier(
                                    "ceil",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.2,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    4,
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
                            FunctionCall(
                                Identifier(
                                    "round",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.5,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    4,
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
                            FunctionCall(
                                Identifier(
                                    "min",
                                ),
                                [
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
                                ],
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
                            FunctionCall(
                                Identifier(
                                    "max",
                                ),
                                [
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
                                ],
                            ),
                            Literal(
                                Int(
                                    5,
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