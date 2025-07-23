# Program
Status: 🟢

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
        Token(Var),
        Token(Ident, "z"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "z"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
        Token(EqualEqual),
        Token(Int, "1"),
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
    Block(
        [
            Declare(
                "z",
                Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            ),
            Function(
                "add",
                [
                    "x",
                    "y",
                ],
                Block(
                    [
                        Assign(
                            "z",
                            Call(
                                GetAttr(
                                    Variable(
                                        "z",
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
                        Call(
                            GetAttr(
                                Variable(
                                    "x",
                                ),
                                "op_add",
                            ),
                            [
                                Variable(
                                    "y",
                                ),
                            ],
                        ),
                    ],
                    None,
                ),
            ),
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
                            Call(
                                Variable(
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
                                "z",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
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