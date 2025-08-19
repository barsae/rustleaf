# Program
Status: 🟢
Assertions: 2

```rustleaf
var z = 0;
fn add(x, y) {
    z += 1;
    x + y;
}
assert(is_unit(add(2, 3)));
assert(z == 1);
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
        1: Token(Ident, "z"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Fn),
        6: Token(Ident, "add"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(Comma),
        10: Token(Ident, "y"),
        11: Token(RightParen),
        12: Token(LeftBrace),
        13: Token(Ident, "z"),
        14: Token(PlusEqual),
        15: Token(Int, "1"),
        16: Token(Semicolon),
        17: Token(Ident, "x"),
        18: Token(Plus),
        19: Token(Ident, "y"),
        20: Token(Semicolon),
        21: Token(RightBrace),
        22: Token(Ident, "assert"),
        23: Token(LeftParen),
        24: Token(Ident, "is_unit"),
        25: Token(LeftParen),
        26: Token(Ident, "add"),
        27: Token(LeftParen),
        28: Token(Int, "2"),
        29: Token(Comma),
        30: Token(Int, "3"),
        31: Token(RightParen),
        32: Token(RightParen),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Ident, "z"),
        38: Token(EqualEqual),
        39: Token(Int, "1"),
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
                    "z",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        Assignment {
                            target: Identifier(
                                "z",
                            ),
                            op: AddAssign,
                            value: Literal(
                                Int(
                                    1,
                                ),
                            ),
                        },
                        Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Identifier(
                                    "y",
                                ),
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
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
                                FunctionCall(
                                    Identifier(
                                        "add",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                        Literal(
                                            Int(
                                                3,
                                            ),
                                        ),
                                    ],
                                ),
                            ],
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
                                "z",
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