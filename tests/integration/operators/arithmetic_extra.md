# Program
Status: 🟢
Assertions: 8

```rustleaf
// Test modulo operator
assert(7 % 3 == 1);
assert(8 % 4 == 0);
assert(7.5 % 2.5 == 0.0);
assert(7 % 2.0 == 1.0);

// Test power operator
assert(2 ** 3 == 8);
assert(2 ** 0 == 1);
assert(3.0 ** 2.0 == 9.0);
assert(2 ** 3.0 == 8.0);
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
        2: Token(Int, "7"),
        3: Token(Percent),
        4: Token(Int, "3"),
        5: Token(EqualEqual),
        6: Token(Int, "1"),
        7: Token(RightParen),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Int, "8"),
        12: Token(Percent),
        13: Token(Int, "4"),
        14: Token(EqualEqual),
        15: Token(Int, "0"),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Float, "7.5"),
        21: Token(Percent),
        22: Token(Float, "2.5"),
        23: Token(EqualEqual),
        24: Token(Float, "0.0"),
        25: Token(RightParen),
        26: Token(Semicolon),
        27: Token(Ident, "assert"),
        28: Token(LeftParen),
        29: Token(Int, "7"),
        30: Token(Percent),
        31: Token(Float, "2.0"),
        32: Token(EqualEqual),
        33: Token(Float, "1.0"),
        34: Token(RightParen),
        35: Token(Semicolon),
        36: Token(Ident, "assert"),
        37: Token(LeftParen),
        38: Token(Int, "2"),
        39: Token(StarStar),
        40: Token(Int, "3"),
        41: Token(EqualEqual),
        42: Token(Int, "8"),
        43: Token(RightParen),
        44: Token(Semicolon),
        45: Token(Ident, "assert"),
        46: Token(LeftParen),
        47: Token(Int, "2"),
        48: Token(StarStar),
        49: Token(Int, "0"),
        50: Token(EqualEqual),
        51: Token(Int, "1"),
        52: Token(RightParen),
        53: Token(Semicolon),
        54: Token(Ident, "assert"),
        55: Token(LeftParen),
        56: Token(Float, "3.0"),
        57: Token(StarStar),
        58: Token(Float, "2.0"),
        59: Token(EqualEqual),
        60: Token(Float, "9.0"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Int, "2"),
        66: Token(StarStar),
        67: Token(Float, "3.0"),
        68: Token(EqualEqual),
        69: Token(Float, "8.0"),
        70: Token(RightParen),
        71: Token(Semicolon),
        72: Token(Eof)
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
                            Mod(
                                Literal(
                                    Int(
                                        7,
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
                            Mod(
                                Literal(
                                    Int(
                                        8,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    0,
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
                            Mod(
                                Literal(
                                    Float(
                                        7.5,
                                    ),
                                ),
                                Literal(
                                    Float(
                                        2.5,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    0.0,
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
                            Mod(
                                Literal(
                                    Int(
                                        7,
                                    ),
                                ),
                                Literal(
                                    Float(
                                        2.0,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    1.0,
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
                            Pow(
                                Literal(
                                    Int(
                                        2,
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
                                    8,
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
                            Pow(
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
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
                            Pow(
                                Literal(
                                    Float(
                                        3.0,
                                    ),
                                ),
                                Literal(
                                    Float(
                                        2.0,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    9.0,
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
                            Pow(
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                                Literal(
                                    Float(
                                        3.0,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    8.0,
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