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
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(While),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "5"),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(RightParen),
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
            Declare(
                "result",
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
                                        5,
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
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "x",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        Variable(
                            "is_unit",
                        ),
                        [
                            Variable(
                                "result",
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```