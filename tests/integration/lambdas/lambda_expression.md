# Program
Status: 🟢
Assertions: 4

```rustleaf
var increment = |x| x + 1;
var double = |y| y * 2;
var add_ten = |z| z + 10;

assert(increment(5) == 6);
assert(double(7) == 14);  
assert(add_ten(15) == 25);
assert(increment(0) == 1);
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
        1: Token(Ident, "increment"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Ident, "x"),
        5: Token(Pipe),
        6: Token(Ident, "x"),
        7: Token(Plus),
        8: Token(Int, "1"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "double"),
        12: Token(Equal),
        13: Token(Pipe),
        14: Token(Ident, "y"),
        15: Token(Pipe),
        16: Token(Ident, "y"),
        17: Token(Star),
        18: Token(Int, "2"),
        19: Token(Semicolon),
        20: Token(Var),
        21: Token(Ident, "add_ten"),
        22: Token(Equal),
        23: Token(Pipe),
        24: Token(Ident, "z"),
        25: Token(Pipe),
        26: Token(Ident, "z"),
        27: Token(Plus),
        28: Token(Int, "10"),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "increment"),
        33: Token(LeftParen),
        34: Token(Int, "5"),
        35: Token(RightParen),
        36: Token(EqualEqual),
        37: Token(Int, "6"),
        38: Token(RightParen),
        39: Token(Semicolon),
        40: Token(Ident, "assert"),
        41: Token(LeftParen),
        42: Token(Ident, "double"),
        43: Token(LeftParen),
        44: Token(Int, "7"),
        45: Token(RightParen),
        46: Token(EqualEqual),
        47: Token(Int, "14"),
        48: Token(RightParen),
        49: Token(Semicolon),
        50: Token(Ident, "assert"),
        51: Token(LeftParen),
        52: Token(Ident, "add_ten"),
        53: Token(LeftParen),
        54: Token(Int, "15"),
        55: Token(RightParen),
        56: Token(EqualEqual),
        57: Token(Int, "25"),
        58: Token(RightParen),
        59: Token(Semicolon),
        60: Token(Ident, "assert"),
        61: Token(LeftParen),
        62: Token(Ident, "increment"),
        63: Token(LeftParen),
        64: Token(Int, "0"),
        65: Token(RightParen),
        66: Token(EqualEqual),
        67: Token(Int, "1"),
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
            VarDecl {
                pattern: Variable(
                    "increment",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "x",
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
            VarDecl {
                pattern: Variable(
                    "double",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "y",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "y",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "add_ten",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "z",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "z",
                                ),
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                            ),
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
                                    "increment",
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
                                    6,
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
                                    "double",
                                ),
                                [
                                    Literal(
                                        Int(
                                            7,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    14,
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
                                    "add_ten",
                                ),
                                [
                                    Literal(
                                        Int(
                                            15,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    25,
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
                                    "increment",
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