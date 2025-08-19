# Program
Status: 🟢
Assertions: 1

```rustleaf
fn add(x, y) { x + y }
assert(add(2, 3) == 5);
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
        1: Token(Ident, "add"),
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
        12: Token(Ident, "assert"),
        13: Token(LeftParen),
        14: Token(Ident, "add"),
        15: Token(LeftParen),
        16: Token(Int, "2"),
        17: Token(Comma),
        18: Token(Int, "3"),
        19: Token(RightParen),
        20: Token(EqualEqual),
        21: Token(Int, "5"),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
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