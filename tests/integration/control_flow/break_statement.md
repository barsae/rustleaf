# Program
Status: 🟢
Assertions: 4

```rustleaf
var result = loop {
    break 42;
};

var result2 = loop {
    break "hello";
};

var counter = 0;
var result3 = loop {
    counter = counter + 1;
    if counter == 3 {
        break counter * 5;
    }
};

assert(result == 42);
assert(result2 == "hello");
assert(result3 == 15);
assert(counter == 3);
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
        1: Token(Ident, "result"),
        2: Token(Equal),
        3: Token(Loop),
        4: Token(LeftBrace),
        5: Token(Break),
        6: Token(Int, "42"),
        7: Token(Semicolon),
        8: Token(RightBrace),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "result2"),
        12: Token(Equal),
        13: Token(Loop),
        14: Token(LeftBrace),
        15: Token(Break),
        16: Token(String, "hello"),
        17: Token(Semicolon),
        18: Token(RightBrace),
        19: Token(Semicolon),
        20: Token(Var),
        21: Token(Ident, "counter"),
        22: Token(Equal),
        23: Token(Int, "0"),
        24: Token(Semicolon),
        25: Token(Var),
        26: Token(Ident, "result3"),
        27: Token(Equal),
        28: Token(Loop),
        29: Token(LeftBrace),
        30: Token(Ident, "counter"),
        31: Token(Equal),
        32: Token(Ident, "counter"),
        33: Token(Plus),
        34: Token(Int, "1"),
        35: Token(Semicolon),
        36: Token(If),
        37: Token(Ident, "counter"),
        38: Token(EqualEqual),
        39: Token(Int, "3"),
        40: Token(LeftBrace),
        41: Token(Break),
        42: Token(Ident, "counter"),
        43: Token(Star),
        44: Token(Int, "5"),
        45: Token(Semicolon),
        46: Token(RightBrace),
        47: Token(RightBrace),
        48: Token(Semicolon),
        49: Token(Ident, "assert"),
        50: Token(LeftParen),
        51: Token(Ident, "result"),
        52: Token(EqualEqual),
        53: Token(Int, "42"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Ident, "assert"),
        57: Token(LeftParen),
        58: Token(Ident, "result2"),
        59: Token(EqualEqual),
        60: Token(String, "hello"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Ident, "result3"),
        66: Token(EqualEqual),
        67: Token(Int, "15"),
        68: Token(RightParen),
        69: Token(Semicolon),
        70: Token(Ident, "assert"),
        71: Token(LeftParen),
        72: Token(Ident, "counter"),
        73: Token(EqualEqual),
        74: Token(Int, "3"),
        75: Token(RightParen),
        76: Token(Semicolon),
        77: Token(Eof)
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
                    "result",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Break(
                                    Some(
                                        Literal(
                                            Int(
                                                42,
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
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Break(
                                    Some(
                                        Literal(
                                            String(
                                                "hello",
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
                    "counter",
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
                    "result3",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "counter",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "counter",
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
                                        condition: Eq(
                                            Identifier(
                                                "counter",
                                            ),
                                            Literal(
                                                Int(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Break(
                                                    Some(
                                                        Mul(
                                                            Identifier(
                                                                "counter",
                                                            ),
                                                            Literal(
                                                                Int(
                                                                    5,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
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
                            Identifier(
                                "result2",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "result3",
                            ),
                            Literal(
                                Int(
                                    15,
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
                                "counter",
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