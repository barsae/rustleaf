# Program
Status: 🟢
Assertions: 4

```rustleaf
// Test lambda expressions in practical usage scenarios

// Simple lambda assignment and calling
var add_one = |x| x + 1;
assert(add_one(5) == 6);

// Lambda with multiple parameters (when supported)
// var add = |x, y| x + y;
// assert(add(3, 4) == 7);

// Lambda as argument to other functions (higher-order functions)
fn apply(func, value) {
    func(value)
}
var double = |x| x * 2;
assert(apply(double, 21) == 42);

// Lambda with block body
var complex_func = |n| {
    var result = n * n;
    result + 1
};
assert(complex_func(3) == 10); // 3*3 + 1 = 10

// Lambda closures - capturing variables from outer scope
var multiplier = 3;
var multiply_by_three = |x| x * multiplier;
assert(multiply_by_three(4) == 12);
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
        1: Token(Ident, "add_one"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Ident, "x"),
        5: Token(Pipe),
        6: Token(Ident, "x"),
        7: Token(Plus),
        8: Token(Int, "1"),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "add_one"),
        13: Token(LeftParen),
        14: Token(Int, "5"),
        15: Token(RightParen),
        16: Token(EqualEqual),
        17: Token(Int, "6"),
        18: Token(RightParen),
        19: Token(Semicolon),
        20: Token(Fn),
        21: Token(Ident, "apply"),
        22: Token(LeftParen),
        23: Token(Ident, "func"),
        24: Token(Comma),
        25: Token(Ident, "value"),
        26: Token(RightParen),
        27: Token(LeftBrace),
        28: Token(Ident, "func"),
        29: Token(LeftParen),
        30: Token(Ident, "value"),
        31: Token(RightParen),
        32: Token(RightBrace),
        33: Token(Var),
        34: Token(Ident, "double"),
        35: Token(Equal),
        36: Token(Pipe),
        37: Token(Ident, "x"),
        38: Token(Pipe),
        39: Token(Ident, "x"),
        40: Token(Star),
        41: Token(Int, "2"),
        42: Token(Semicolon),
        43: Token(Ident, "assert"),
        44: Token(LeftParen),
        45: Token(Ident, "apply"),
        46: Token(LeftParen),
        47: Token(Ident, "double"),
        48: Token(Comma),
        49: Token(Int, "21"),
        50: Token(RightParen),
        51: Token(EqualEqual),
        52: Token(Int, "42"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Var),
        56: Token(Ident, "complex_func"),
        57: Token(Equal),
        58: Token(Pipe),
        59: Token(Ident, "n"),
        60: Token(Pipe),
        61: Token(LeftBrace),
        62: Token(Var),
        63: Token(Ident, "result"),
        64: Token(Equal),
        65: Token(Ident, "n"),
        66: Token(Star),
        67: Token(Ident, "n"),
        68: Token(Semicolon),
        69: Token(Ident, "result"),
        70: Token(Plus),
        71: Token(Int, "1"),
        72: Token(RightBrace),
        73: Token(Semicolon),
        74: Token(Ident, "assert"),
        75: Token(LeftParen),
        76: Token(Ident, "complex_func"),
        77: Token(LeftParen),
        78: Token(Int, "3"),
        79: Token(RightParen),
        80: Token(EqualEqual),
        81: Token(Int, "10"),
        82: Token(RightParen),
        83: Token(Semicolon),
        84: Token(Var),
        85: Token(Ident, "multiplier"),
        86: Token(Equal),
        87: Token(Int, "3"),
        88: Token(Semicolon),
        89: Token(Var),
        90: Token(Ident, "multiply_by_three"),
        91: Token(Equal),
        92: Token(Pipe),
        93: Token(Ident, "x"),
        94: Token(Pipe),
        95: Token(Ident, "x"),
        96: Token(Star),
        97: Token(Ident, "multiplier"),
        98: Token(Semicolon),
        99: Token(Ident, "assert"),
        100: Token(LeftParen),
        101: Token(Ident, "multiply_by_three"),
        102: Token(LeftParen),
        103: Token(Int, "4"),
        104: Token(RightParen),
        105: Token(EqualEqual),
        106: Token(Int, "12"),
        107: Token(RightParen),
        108: Token(Semicolon),
        109: Token(Eof)
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
                    "add_one",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "add_one",
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
            FnDecl {
                name: "apply",
                params: [
                    Parameter {
                        name: "func",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "value",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        FunctionCall(
                            Identifier(
                                "func",
                            ),
                            [
                                Identifier(
                                    "value",
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "double",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
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
                                    "apply",
                                ),
                                [
                                    Identifier(
                                        "double",
                                    ),
                                    Literal(
                                        Int(
                                            21,
                                        ),
                                    ),
                                ],
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
            VarDecl {
                pattern: Variable(
                    "complex_func",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "n",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "result",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "n",
                                                ),
                                                Identifier(
                                                    "n",
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Add(
                                        Identifier(
                                            "result",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "complex_func",
                                ),
                                [
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "multiplier",
                ),
                value: Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "multiply_by_three",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "x",
                                ),
                                Identifier(
                                    "multiplier",
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
                                    "multiply_by_three",
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
                                Int(
                                    12,
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