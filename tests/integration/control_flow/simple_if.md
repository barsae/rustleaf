# Program
Status: 🟢
Assertions: 3

```rustleaf
var result1 = if true { 42 } else { 0 };
var result2 = if false { 100 } else { 200 };
var x = 5;
var result3 = if x > 3 { "big" } else { "small" };
assert(result1 == 42);
assert(result2 == 200);
assert(result3 == "big");
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
        1: Token(Ident, "result1"),
        2: Token(Equal),
        3: Token(If),
        4: Token(True),
        5: Token(LeftBrace),
        6: Token(Int, "42"),
        7: Token(RightBrace),
        8: Token(Else),
        9: Token(LeftBrace),
        10: Token(Int, "0"),
        11: Token(RightBrace),
        12: Token(Semicolon),
        13: Token(Var),
        14: Token(Ident, "result2"),
        15: Token(Equal),
        16: Token(If),
        17: Token(False),
        18: Token(LeftBrace),
        19: Token(Int, "100"),
        20: Token(RightBrace),
        21: Token(Else),
        22: Token(LeftBrace),
        23: Token(Int, "200"),
        24: Token(RightBrace),
        25: Token(Semicolon),
        26: Token(Var),
        27: Token(Ident, "x"),
        28: Token(Equal),
        29: Token(Int, "5"),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "result3"),
        33: Token(Equal),
        34: Token(If),
        35: Token(Ident, "x"),
        36: Token(Greater),
        37: Token(Int, "3"),
        38: Token(LeftBrace),
        39: Token(String, "big"),
        40: Token(RightBrace),
        41: Token(Else),
        42: Token(LeftBrace),
        43: Token(String, "small"),
        44: Token(RightBrace),
        45: Token(Semicolon),
        46: Token(Ident, "assert"),
        47: Token(LeftParen),
        48: Token(Ident, "result1"),
        49: Token(EqualEqual),
        50: Token(Int, "42"),
        51: Token(RightParen),
        52: Token(Semicolon),
        53: Token(Ident, "assert"),
        54: Token(LeftParen),
        55: Token(Ident, "result2"),
        56: Token(EqualEqual),
        57: Token(Int, "200"),
        58: Token(RightParen),
        59: Token(Semicolon),
        60: Token(Ident, "assert"),
        61: Token(LeftParen),
        62: Token(Ident, "result3"),
        63: Token(EqualEqual),
        64: Token(String, "big"),
        65: Token(RightParen),
        66: Token(Semicolon),
        67: Token(Eof)
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
                    "result1",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                true,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            0,
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
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                false,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            200,
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "big",
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        String(
                                            "small",
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
                            Identifier(
                                "result1",
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
                                Int(
                                    200,
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
                                String(
                                    "big",
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