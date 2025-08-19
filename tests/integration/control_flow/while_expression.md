# Program
Status: 🟢
Assertions: 2

```rustleaf
// Test while loop as expression with proper variable initialization
var x = 0;
var result = while x < 5 {
    x = x + 1;
};
assert(x == 5);
assert(is_unit(result));
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
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(While),
        9: Token(Ident, "x"),
        10: Token(Less),
        11: Token(Int, "5"),
        12: Token(LeftBrace),
        13: Token(Ident, "x"),
        14: Token(Equal),
        15: Token(Ident, "x"),
        16: Token(Plus),
        17: Token(Int, "1"),
        18: Token(Semicolon),
        19: Token(RightBrace),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "x"),
        24: Token(EqualEqual),
        25: Token(Int, "5"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "is_unit"),
        31: Token(LeftParen),
        32: Token(Ident, "result"),
        33: Token(RightParen),
        34: Token(RightParen),
        35: Token(Semicolon),
        36: Token(Eof)
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
                    While {
                        condition: Lt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ),
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "x",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "x",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
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
                                "x",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
                                Identifier(
                                    "result",
                                ),
                            ],
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