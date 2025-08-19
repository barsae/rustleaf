# Program
Status: 🟢
Assertions: 4

```rustleaf
var x = 1;
var result = if x > 0 {
    "positive"
} else {
    "zero or negative"
};

var y = -5;
var result2 = if y > 0 {
    "positive"
} else {
    "zero or negative"
};

assert(result == "positive");
assert(result2 == "zero or negative");
assert(x == 1);
assert(y == -5);
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "1"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(If),
        9: Token(Ident, "x"),
        10: Token(Greater),
        11: Token(Int, "0"),
        12: Token(LeftBrace),
        13: Token(String, "positive"),
        14: Token(RightBrace),
        15: Token(Else),
        16: Token(LeftBrace),
        17: Token(String, "zero or negative"),
        18: Token(RightBrace),
        19: Token(Semicolon),
        20: Token(Var),
        21: Token(Ident, "y"),
        22: Token(Equal),
        23: Token(Minus),
        24: Token(Int, "5"),
        25: Token(Semicolon),
        26: Token(Var),
        27: Token(Ident, "result2"),
        28: Token(Equal),
        29: Token(If),
        30: Token(Ident, "y"),
        31: Token(Greater),
        32: Token(Int, "0"),
        33: Token(LeftBrace),
        34: Token(String, "positive"),
        35: Token(RightBrace),
        36: Token(Else),
        37: Token(LeftBrace),
        38: Token(String, "zero or negative"),
        39: Token(RightBrace),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Ident, "result"),
        44: Token(EqualEqual),
        45: Token(String, "positive"),
        46: Token(RightParen),
        47: Token(Semicolon),
        48: Token(Ident, "assert"),
        49: Token(LeftParen),
        50: Token(Ident, "result2"),
        51: Token(EqualEqual),
        52: Token(String, "zero or negative"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Ident, "assert"),
        56: Token(LeftParen),
        57: Token(Ident, "x"),
        58: Token(EqualEqual),
        59: Token(Int, "1"),
        60: Token(RightParen),
        61: Token(Semicolon),
        62: Token(Ident, "assert"),
        63: Token(LeftParen),
        64: Token(Ident, "y"),
        65: Token(EqualEqual),
        66: Token(Minus),
        67: Token(Int, "5"),
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
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
                    "y",
                ),
                value: Some(
                    Neg(
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "y",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "positive",
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
                                    "zero or negative",
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
                                "x",
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
                            Identifier(
                                "y",
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        5,
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