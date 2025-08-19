# Program
Status: 🟢
Assertions: 1

```rustleaf
fn test(x, y) {
    x + y
}

var z = 1 | test(2);
assert(z == 3);
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
        1: Token(Ident, "test"),
        2: Token(LeftParen),
        3: Token(Ident, "x"),
        4: Token(Comma),
        5: Token(Ident, "y"),
        6: Token(RightParen),
        7: Token(LeftBrace),
        8: Token(Ident, "x"),
        9: Token(Plus),
        10: Token(Ident, "y"),
        11: Token(RightBrace),
        12: Token(Var),
        13: Token(Ident, "z"),
        14: Token(Equal),
        15: Token(Int, "1"),
        16: Token(Pipe),
        17: Token(Ident, "test"),
        18: Token(LeftParen),
        19: Token(Int, "2"),
        20: Token(RightParen),
        21: Token(Semicolon),
        22: Token(Ident, "assert"),
        23: Token(LeftParen),
        24: Token(Ident, "z"),
        25: Token(EqualEqual),
        26: Token(Int, "3"),
        27: Token(RightParen),
        28: Token(Semicolon),
        29: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "test",
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
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: Some(
                    Pipe(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        FunctionCall(
                            Identifier(
                                "test",
                            ),
                            [
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ],
                        ),
                    ),
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
                                "z",
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