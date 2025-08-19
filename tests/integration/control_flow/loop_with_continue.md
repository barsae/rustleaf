# Program
Status: 🟢
Assertions: 5

```rustleaf
var i = 0;
var result = loop {
    i = i + 1;
    if i < 3 {
        continue;
    }
    break i * 10;
};

var j = 0;
var count = 0;
var result2 = loop {
    j = j + 1;
    if j <= 5 {
        count = count + 1;
        continue;
    }
    break j + count;
};

assert(result == 30);
assert(i == 3);
assert(result2 == 11);  
assert(j == 6);
assert(count == 5);
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
        1: Token(Ident, "i"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(Loop),
        9: Token(LeftBrace),
        10: Token(Ident, "i"),
        11: Token(Equal),
        12: Token(Ident, "i"),
        13: Token(Plus),
        14: Token(Int, "1"),
        15: Token(Semicolon),
        16: Token(If),
        17: Token(Ident, "i"),
        18: Token(Less),
        19: Token(Int, "3"),
        20: Token(LeftBrace),
        21: Token(Continue),
        22: Token(Semicolon),
        23: Token(RightBrace),
        24: Token(Break),
        25: Token(Ident, "i"),
        26: Token(Star),
        27: Token(Int, "10"),
        28: Token(Semicolon),
        29: Token(RightBrace),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "j"),
        33: Token(Equal),
        34: Token(Int, "0"),
        35: Token(Semicolon),
        36: Token(Var),
        37: Token(Ident, "count"),
        38: Token(Equal),
        39: Token(Int, "0"),
        40: Token(Semicolon),
        41: Token(Var),
        42: Token(Ident, "result2"),
        43: Token(Equal),
        44: Token(Loop),
        45: Token(LeftBrace),
        46: Token(Ident, "j"),
        47: Token(Equal),
        48: Token(Ident, "j"),
        49: Token(Plus),
        50: Token(Int, "1"),
        51: Token(Semicolon),
        52: Token(If),
        53: Token(Ident, "j"),
        54: Token(LessEqual),
        55: Token(Int, "5"),
        56: Token(LeftBrace),
        57: Token(Ident, "count"),
        58: Token(Equal),
        59: Token(Ident, "count"),
        60: Token(Plus),
        61: Token(Int, "1"),
        62: Token(Semicolon),
        63: Token(Continue),
        64: Token(Semicolon),
        65: Token(RightBrace),
        66: Token(Break),
        67: Token(Ident, "j"),
        68: Token(Plus),
        69: Token(Ident, "count"),
        70: Token(Semicolon),
        71: Token(RightBrace),
        72: Token(Semicolon),
        73: Token(Ident, "assert"),
        74: Token(LeftParen),
        75: Token(Ident, "result"),
        76: Token(EqualEqual),
        77: Token(Int, "30"),
        78: Token(RightParen),
        79: Token(Semicolon),
        80: Token(Ident, "assert"),
        81: Token(LeftParen),
        82: Token(Ident, "i"),
        83: Token(EqualEqual),
        84: Token(Int, "3"),
        85: Token(RightParen),
        86: Token(Semicolon),
        87: Token(Ident, "assert"),
        88: Token(LeftParen),
        89: Token(Ident, "result2"),
        90: Token(EqualEqual),
        91: Token(Int, "11"),
        92: Token(RightParen),
        93: Token(Semicolon),
        94: Token(Ident, "assert"),
        95: Token(LeftParen),
        96: Token(Ident, "j"),
        97: Token(EqualEqual),
        98: Token(Int, "6"),
        99: Token(RightParen),
        100: Token(Semicolon),
        101: Token(Ident, "assert"),
        102: Token(LeftParen),
        103: Token(Ident, "count"),
        104: Token(EqualEqual),
        105: Token(Int, "5"),
        106: Token(RightParen),
        107: Token(Semicolon),
        108: Token(Eof)
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
                    "i",
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
                    "result",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "i",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "i",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Lt(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Continue,
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                                Break(
                                    Some(
                                        Mul(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    10,
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "j",
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
                    "count",
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
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "j",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "j",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Le(
                                            Identifier(
                                                "j",
                                            ),
                                            Literal(
                                                Int(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Assignment {
                                                    target: Identifier(
                                                        "count",
                                                    ),
                                                    op: Assign,
                                                    value: Add(
                                                        Identifier(
                                                            "count",
                                                        ),
                                                        Literal(
                                                            Int(
                                                                1,
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Continue,
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                                Break(
                                    Some(
                                        Add(
                                            Identifier(
                                                "j",
                                            ),
                                            Identifier(
                                                "count",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
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
                            Identifier(
                                "result",
                            ),
                            Literal(
                                Int(
                                    30,
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
                                "i",
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
                            Identifier(
                                "result2",
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
                            Identifier(
                                "j",
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
                            Identifier(
                                "count",
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