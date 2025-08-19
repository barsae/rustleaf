# Program
Status: 🟢
Assertions: 4

```rustleaf
var num = 123;
var zero = 0;
var negative = -42;
assert(num == 123);
assert(zero == 0);
assert(negative == -42);
assert(num + zero == 123);
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
        1: Token(Ident, "num"),
        2: Token(Equal),
        3: Token(Int, "123"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "zero"),
        7: Token(Equal),
        8: Token(Int, "0"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "negative"),
        12: Token(Equal),
        13: Token(Minus),
        14: Token(Int, "42"),
        15: Token(Semicolon),
        16: Token(Ident, "assert"),
        17: Token(LeftParen),
        18: Token(Ident, "num"),
        19: Token(EqualEqual),
        20: Token(Int, "123"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Ident, "assert"),
        24: Token(LeftParen),
        25: Token(Ident, "zero"),
        26: Token(EqualEqual),
        27: Token(Int, "0"),
        28: Token(RightParen),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "negative"),
        33: Token(EqualEqual),
        34: Token(Minus),
        35: Token(Int, "42"),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Ident, "assert"),
        39: Token(LeftParen),
        40: Token(Ident, "num"),
        41: Token(Plus),
        42: Token(Ident, "zero"),
        43: Token(EqualEqual),
        44: Token(Int, "123"),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Eof)
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
                    "num",
                ),
                value: Some(
                    Literal(
                        Int(
                            123,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "zero",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
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
                            Int(
                                42,
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
                                "num",
                            ),
                            Literal(
                                Int(
                                    123,
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
                                "zero",
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
                            Add(
                                Identifier(
                                    "num",
                                ),
                                Identifier(
                                    "zero",
                                ),
                            ),
                            Literal(
                                Int(
                                    123,
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