# Program
Status: 🟢
Assertions: 4

```rustleaf
var processor = |x| {
    var temp = x * 2;
    temp + 1
};

var complex_lambda = |y| {
    var first = y + 10;
    var second = first * 3;
    second - 5
};

assert(processor(5) == 11);
assert(processor(0) == 1);
assert(complex_lambda(2) == 31);
assert(complex_lambda(10) == 55);
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
        1: Token(Ident, "processor"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Ident, "x"),
        5: Token(Pipe),
        6: Token(LeftBrace),
        7: Token(Var),
        8: Token(Ident, "temp"),
        9: Token(Equal),
        10: Token(Ident, "x"),
        11: Token(Star),
        12: Token(Int, "2"),
        13: Token(Semicolon),
        14: Token(Ident, "temp"),
        15: Token(Plus),
        16: Token(Int, "1"),
        17: Token(RightBrace),
        18: Token(Semicolon),
        19: Token(Var),
        20: Token(Ident, "complex_lambda"),
        21: Token(Equal),
        22: Token(Pipe),
        23: Token(Ident, "y"),
        24: Token(Pipe),
        25: Token(LeftBrace),
        26: Token(Var),
        27: Token(Ident, "first"),
        28: Token(Equal),
        29: Token(Ident, "y"),
        30: Token(Plus),
        31: Token(Int, "10"),
        32: Token(Semicolon),
        33: Token(Var),
        34: Token(Ident, "second"),
        35: Token(Equal),
        36: Token(Ident, "first"),
        37: Token(Star),
        38: Token(Int, "3"),
        39: Token(Semicolon),
        40: Token(Ident, "second"),
        41: Token(Minus),
        42: Token(Int, "5"),
        43: Token(RightBrace),
        44: Token(Semicolon),
        45: Token(Ident, "assert"),
        46: Token(LeftParen),
        47: Token(Ident, "processor"),
        48: Token(LeftParen),
        49: Token(Int, "5"),
        50: Token(RightParen),
        51: Token(EqualEqual),
        52: Token(Int, "11"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Ident, "assert"),
        56: Token(LeftParen),
        57: Token(Ident, "processor"),
        58: Token(LeftParen),
        59: Token(Int, "0"),
        60: Token(RightParen),
        61: Token(EqualEqual),
        62: Token(Int, "1"),
        63: Token(RightParen),
        64: Token(Semicolon),
        65: Token(Ident, "assert"),
        66: Token(LeftParen),
        67: Token(Ident, "complex_lambda"),
        68: Token(LeftParen),
        69: Token(Int, "2"),
        70: Token(RightParen),
        71: Token(EqualEqual),
        72: Token(Int, "31"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Ident, "assert"),
        76: Token(LeftParen),
        77: Token(Ident, "complex_lambda"),
        78: Token(LeftParen),
        79: Token(Int, "10"),
        80: Token(RightParen),
        81: Token(EqualEqual),
        82: Token(Int, "55"),
        83: Token(RightParen),
        84: Token(Semicolon),
        85: Token(Eof)
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
                    "processor",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "temp",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "x",
                                                ),
                                                Literal(
                                                    Int(
                                                        2,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Add(
                                        Identifier(
                                            "temp",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "complex_lambda",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "y",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "first",
                                        ),
                                        value: Some(
                                            Add(
                                                Identifier(
                                                    "y",
                                                ),
                                                Literal(
                                                    Int(
                                                        10,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    VarDecl {
                                        pattern: Variable(
                                            "second",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "first",
                                                ),
                                                Literal(
                                                    Int(
                                                        3,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Sub(
                                        Identifier(
                                            "second",
                                        ),
                                        Literal(
                                            Int(
                                                5,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "processor",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    11,
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
                                    "processor",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ],
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
                            FunctionCall(
                                Identifier(
                                    "complex_lambda",
                                ),
                                [
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    31,
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
                                    "complex_lambda",
                                ),
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    55,
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