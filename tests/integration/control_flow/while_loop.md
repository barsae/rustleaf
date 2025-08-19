# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = 0;
var count = 0;
while x < 3 {
    count = count + 1;
    x = x + 1;
}
assert(x == 3);
assert(count == 3);
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
        6: Token(Ident, "count"),
        7: Token(Equal),
        8: Token(Int, "0"),
        9: Token(Semicolon),
        10: Token(While),
        11: Token(Ident, "x"),
        12: Token(Less),
        13: Token(Int, "3"),
        14: Token(LeftBrace),
        15: Token(Ident, "count"),
        16: Token(Equal),
        17: Token(Ident, "count"),
        18: Token(Plus),
        19: Token(Int, "1"),
        20: Token(Semicolon),
        21: Token(Ident, "x"),
        22: Token(Equal),
        23: Token(Ident, "x"),
        24: Token(Plus),
        25: Token(Int, "1"),
        26: Token(Semicolon),
        27: Token(RightBrace),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "x"),
        31: Token(EqualEqual),
        32: Token(Int, "3"),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Ident, "count"),
        38: Token(EqualEqual),
        39: Token(Int, "3"),
        40: Token(RightParen),
        41: Token(Semicolon),
        42: Token(Eof)
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
            Expression(
                While {
                    condition: Lt(
                        Identifier(
                            "x",
                        ),
                        Literal(
                            Int(
                                3,
                            ),
                        ),
                    ),
                    body: Block {
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
                                "count",
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