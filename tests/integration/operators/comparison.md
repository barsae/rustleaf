# Program
Status: 🟢
Assertions: 10

```rustleaf
// Basic comparison tests
assert(3 == 3);
assert(3 != 4);
assert(3 < 4);
assert(4 > 3);
assert(3 <= 3);
assert(3 >= 3);

// Mixed numeric comparisons
assert(3 == 3.0);
assert(3.5 > 3);

// String comparisons
assert("a" < "b");
assert("hello" == "hello");
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
        2: Token(Int, "3"),
        3: Token(EqualEqual),
        4: Token(Int, "3"),
        5: Token(RightParen),
        6: Token(Semicolon),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Int, "3"),
        10: Token(BangEqual),
        11: Token(Int, "4"),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Int, "3"),
        17: Token(Less),
        18: Token(Int, "4"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Int, "4"),
        24: Token(Greater),
        25: Token(Int, "3"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Int, "3"),
        31: Token(LessEqual),
        32: Token(Int, "3"),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Int, "3"),
        38: Token(GreaterEqual),
        39: Token(Int, "3"),
        40: Token(RightParen),
        41: Token(Semicolon),
        42: Token(Ident, "assert"),
        43: Token(LeftParen),
        44: Token(Int, "3"),
        45: Token(EqualEqual),
        46: Token(Float, "3.0"),
        47: Token(RightParen),
        48: Token(Semicolon),
        49: Token(Ident, "assert"),
        50: Token(LeftParen),
        51: Token(Float, "3.5"),
        52: Token(Greater),
        53: Token(Int, "3"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Ident, "assert"),
        57: Token(LeftParen),
        58: Token(String, "a"),
        59: Token(Less),
        60: Token(String, "b"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(String, "hello"),
        66: Token(EqualEqual),
        67: Token(String, "hello"),
        68: Token(RightParen),
        69: Token(Semicolon),
        70: Token(Eof)
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
                                Int(
                                    3,
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
                        Ne(
                            Literal(
                                Int(
                                    3,
                                ),
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
                        Lt(
                            Literal(
                                Int(
                                    3,
                                ),
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
                        Gt(
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
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Le(
                            Literal(
                                Int(
                                    3,
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
                        Ge(
                            Literal(
                                Int(
                                    3,
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
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Float(
                                    3.0,
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
                        Gt(
                            Literal(
                                Float(
                                    3.5,
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
                        Lt(
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Literal(
                                String(
                                    "b",
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
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello",
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