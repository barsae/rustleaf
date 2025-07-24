# Program
Status: 🟢
Assertions: 0

```rustleaf
var i = 0;
print(loop {
    i = i + 1;
    if i < 3 {
        continue;
    }
    break i * 10;
});
```

# Output
```
Int(30)
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
        Token(Ident, "i"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Equal),
        Token(Ident, "i"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "i"),
        Token(Less),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Ident, "i"),
        Token(Star),
        Token(Int, "10"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightParen),
        Token(Semicolon),
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
                    "i",
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
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Loop {
                            body: Block {
                                statements: [
                                    Assignment {
                                        target: Identifier(
                                            "i",
                                        ),
                                        op: Assign,
                                        value: Add(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    },
                                    Expression(
                                        If {
                                            condition: Lt(
                                                Identifier(
                                                    "i",
                                                ),
                                                Literal(
                                                    Int(
                                                        3,
                                                    ),
                                                ),
                                            ),
                                            then_expr: Block {
                                                statements: [
                                                    Continue,
                                                ],
                                                final_expr: None,
                                            },
                                            else_expr: None,
                                        },
                                    ),
                                    Break(
                                        Some(
                                            Mul(
                                                Identifier(
                                                    "i",
                                                ),
                                                Literal(
                                                    Int(
                                                        10,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ],
                                final_expr: None,
                            },
                        },
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
    Block(
        [
            Declare(
                "i",
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
            Call(
                Variable(
                    "print",
                ),
                [
                    Loop(
                        Block(
                            [
                                Assign(
                                    "i",
                                    Call(
                                        GetAttr(
                                            Variable(
                                                "i",
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
                                If(
                                    Call(
                                        GetAttr(
                                            Variable(
                                                "i",
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
                                            Continue,
                                        ],
                                        None,
                                    ),
                                    None,
                                ),
                                Break(
                                    Some(
                                        Call(
                                            GetAttr(
                                                Variable(
                                                    "i",
                                                ),
                                                "op_mul",
                                            ),
                                            [
                                                Literal(
                                                    Int(
                                                        10,
                                                    ),
                                                ),
                                            ],
                                        ),
                                    ),
                                ),
                            ],
                            None,
                        ),
                    ),
                ],
            ),
        ),
    ),
)
```