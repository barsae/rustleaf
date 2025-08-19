# Program
Status: 🟢
Assertions: 4

```rustleaf
var positive = 42;
var negative = -positive;
var double_neg = -negative;
assert(negative == -42);
assert(double_neg == 42);
assert(-100 == -100);
assert(-(5 + 3) == -8);
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
        1: Token(Ident, "positive"),
        2: Token(Equal),
        3: Token(Int, "42"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "negative"),
        7: Token(Equal),
        8: Token(Minus),
        9: Token(Ident, "positive"),
        10: Token(Semicolon),
        11: Token(Var),
        12: Token(Ident, "double_neg"),
        13: Token(Equal),
        14: Token(Minus),
        15: Token(Ident, "negative"),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "negative"),
        20: Token(EqualEqual),
        21: Token(Minus),
        22: Token(Int, "42"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Ident, "assert"),
        26: Token(LeftParen),
        27: Token(Ident, "double_neg"),
        28: Token(EqualEqual),
        29: Token(Int, "42"),
        30: Token(RightParen),
        31: Token(Semicolon),
        32: Token(Ident, "assert"),
        33: Token(LeftParen),
        34: Token(Minus),
        35: Token(Int, "100"),
        36: Token(EqualEqual),
        37: Token(Minus),
        38: Token(Int, "100"),
        39: Token(RightParen),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Minus),
        44: Token(LeftParen),
        45: Token(Int, "5"),
        46: Token(Plus),
        47: Token(Int, "3"),
        48: Token(RightParen),
        49: Token(EqualEqual),
        50: Token(Minus),
        51: Token(Int, "8"),
        52: Token(RightParen),
        53: Token(Semicolon),
        54: Token(Eof)
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
                    "positive",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
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
                        Identifier(
                            "positive",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "double_neg",
                ),
                value: Some(
                    Neg(
                        Identifier(
                            "negative",
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
                                "negative",
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        42,
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
                            Identifier(
                                "double_neg",
                            ),
                            Literal(
                                Int(
                                    42,
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
                            Neg(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        100,
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
                            Neg(
                                Add(
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
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        8,
                                    ),
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