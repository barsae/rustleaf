# Program 🟢
```rustleaf
var s = 0;
for x in [1, 2, 3] {
    s += x;
}
assert(s == 6);
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
        Token(Ident, "s"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "x"),
        Token(In),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(LeftBrace),
        Token(Ident, "s"),
        Token(PlusEqual),
        Token(Ident, "x"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "s"),
        Token(EqualEqual),
        Token(Int, "6"),
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
                    "s",
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
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: List(
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
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "s",
                                ),
                                op: AddAssign,
                                value: Identifier(
                                    "x",
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
                                "s",
                            ),
                            Literal(
                                Int(
                                    6,
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
    Block(
        [
            Declare(
                "s",
                Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            ),
            For(
                "x",
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
                Block(
                    [
                        Assign(
                            "s",
                            Call(
                                GetAttr(
                                    Variable(
                                        "s",
                                    ),
                                    "op_add",
                                ),
                                [
                                    Variable(
                                        "x",
                                    ),
                                ],
                            ),
                        ),
                    ],
                    None,
                ),
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "s",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    6,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```