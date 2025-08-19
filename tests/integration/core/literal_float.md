# Program
Status: 🟢
Assertions: 5

```rustleaf
var pi = 3.14;
var small = 0.1;
var negative = -2.5;
assert(pi == 3.14);
assert(small == 0.1);
assert(negative == -2.5);
assert(pi + small == 3.24);
assert(pi * 2.0 == 6.28);
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
        0: Token(Var),
        1: Token(Ident, "pi"),
        2: Token(Equal),
        3: Token(Float, "3.14"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "small"),
        7: Token(Equal),
        8: Token(Float, "0.1"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "negative"),
        12: Token(Equal),
        13: Token(Minus),
        14: Token(Float, "2.5"),
        15: Token(Semicolon),
        16: Token(Ident, "assert"),
        17: Token(LeftParen),
        18: Token(Ident, "pi"),
        19: Token(EqualEqual),
        20: Token(Float, "3.14"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Ident, "assert"),
        24: Token(LeftParen),
        25: Token(Ident, "small"),
        26: Token(EqualEqual),
        27: Token(Float, "0.1"),
        28: Token(RightParen),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "negative"),
        33: Token(EqualEqual),
        34: Token(Minus),
        35: Token(Float, "2.5"),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Ident, "assert"),
        39: Token(LeftParen),
        40: Token(Ident, "pi"),
        41: Token(Plus),
        42: Token(Ident, "small"),
        43: Token(EqualEqual),
        44: Token(Float, "3.24"),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Ident, "assert"),
        48: Token(LeftParen),
        49: Token(Ident, "pi"),
        50: Token(Star),
        51: Token(Float, "2.0"),
        52: Token(EqualEqual),
        53: Token(Float, "6.28"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: Variable(
                    "pi",
                ),
                value: Some(
                    Literal(
                        Float(
                            3.14,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "small",
                ),
                value: Some(
                    Literal(
                        Float(
                            0.1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "negative",
                ),
                value: Some(
                    Neg(
                        Literal(
                            Float(
                                2.5,
                            ),
                        ),
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "pi",
                            ),
                            Literal(
                                Float(
                                    3.14,
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
                            Identifier(
                                "small",
                            ),
                            Literal(
                                Float(
                                    0.1,
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
                            Identifier(
                                "negative",
                            ),
                            Neg(
                                Literal(
                                    Float(
                                        2.5,
                                    ),
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
                                Identifier(
                                    "pi",
                                ),
                                Identifier(
                                    "small",
                                ),
                            ),
                            Literal(
                                Float(
                                    3.24,
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
                                Identifier(
                                    "pi",
                                ),
                                Literal(
                                    Float(
                                        2.0,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    6.28,
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