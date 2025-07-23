# Program 🟢
```rustleaf
var x = 1;
loop {
    x += 1;
    if (x < 2) {
        continue;
    }
    break;
}
assert(x == 2);
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
        Token(Int, "1"),
        Token(Semicolon),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "2"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "2"),
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
                            1,
                        ),
                    ),
                ),
            },
            Expression(
                Loop {
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            },
                            Expression(
                                If {
                                    condition: Lt(
                                        Identifier(
                                            "x",
                                        ),
                                        Literal(
                                            Int(
                                                2,
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
                                None,
                            ),
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
                                    2,
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
                "x",
                Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            ),
            Loop(
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
                        If(
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
                                            2,
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
                            None,
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
                                "x",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    2,
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