# Program 🔴
```rustleaf
// #[fail_quietly]
while x < 10 {
    x = x + 1;
}
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: x",
)
```

# Lex
```rust
Ok(
    [
        Token(While),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "10"),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Eof),
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Expression(
                While {
                    condition: Lt(
                        Identifier(
                            "x",
                        ),
                        Literal(
                            Int(
                                10,
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
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [],
        Some(
            While(
                Call(
                    GetAttr(
                        Variable(
                            "x",
                        ),
                        "op_lt",
                    ),
                    [
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                    ],
                ),
                Block(
                    [
                        Assign(
                            "x",
                            Call(
                                GetAttr(
                                    Variable(
                                        "x",
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ],
                            ),
                        ),
                    ],
                    None,
                ),
            ),
        ),
    ),
)
```