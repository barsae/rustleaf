# Program
Status: 🟢

```rustleaf
var x = 0;
while x < 3 {
    print(x);
    x = x + 1;
}
```

# Output
```
Int(0)
Int(1)
Int(2)
```

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
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(While),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(RightParen),
        Token(Semicolon),
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
                            Expression(
                                FunctionCall(
                                    Identifier(
                                        "print",
                                    ),
                                    [
                                        Identifier(
                                            "x",
                                        ),
                                    ],
                                ),
                            ),
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
        [
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            ),
        ],
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
                                3,
                            ),
                        ),
                    ],
                ),
                Block(
                    [
                        Call(
                            Variable(
                                "print",
                            ),
                            [
                                Variable(
                                    "x",
                                ),
                            ],
                        ),
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