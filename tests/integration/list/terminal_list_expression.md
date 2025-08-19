# Program
Status: 🟢
Assertions: 1

```rustleaf
fn f() {
    var j = [10, 20];
    for i in j {
        i;
    }
    [1, 2, 3]
}
// This used to parse as "for_loop[1, 2, 3]", an indexing operation
assert(f() == [1, 2, 3]);
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
        0: Token(Fn),
        1: Token(Ident, "f"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Var),
        6: Token(Ident, "j"),
        7: Token(Equal),
        8: Token(LeftBracket),
        9: Token(Int, "10"),
        10: Token(Comma),
        11: Token(Int, "20"),
        12: Token(RightBracket),
        13: Token(Semicolon),
        14: Token(For),
        15: Token(Ident, "i"),
        16: Token(In),
        17: Token(Ident, "j"),
        18: Token(LeftBrace),
        19: Token(Ident, "i"),
        20: Token(Semicolon),
        21: Token(RightBrace),
        22: Token(LeftBracket),
        23: Token(Int, "1"),
        24: Token(Comma),
        25: Token(Int, "2"),
        26: Token(Comma),
        27: Token(Int, "3"),
        28: Token(RightBracket),
        29: Token(RightBrace),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "f"),
        33: Token(LeftParen),
        34: Token(RightParen),
        35: Token(EqualEqual),
        36: Token(LeftBracket),
        37: Token(Int, "1"),
        38: Token(Comma),
        39: Token(Int, "2"),
        40: Token(Comma),
        41: Token(Int, "3"),
        42: Token(RightBracket),
        43: Token(RightParen),
        44: Token(Semicolon),
        45: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "f",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "j",
                            ),
                            value: Some(
                                List(
                                    [
                                        Literal(
                                            Int(
                                                10,
                                            ),
                                        ),
                                        Literal(
                                            Int(
                                                20,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        },
                        Expression(
                            For {
                                pattern: Variable(
                                    "i",
                                ),
                                iter: Identifier(
                                    "j",
                                ),
                                body: Block {
                                    statements: [
                                        Expression(
                                            Identifier(
                                                "i",
                                            ),
                                        ),
                                    ],
                                    final_expr: None,
                                },
                            },
                        ),
                    ],
                    final_expr: Some(
                        List(
                            [
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
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
                    ),
                },
                is_pub: false,
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
                                    "f",
                                ),
                                [],
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
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