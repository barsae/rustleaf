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
        Token(Fn),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "j"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "10"),
        Token(Comma),
        Token(Int, "20"),
        Token(RightBracket),
        Token(Semicolon),
        Token(For),
        Token(Ident, "i"),
        Token(In),
        Token(Ident, "j"),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Semicolon),
        Token(RightBrace),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "f",
                            params: [],
                            body: RustValue(
                                EvalBlock {
                                    statements: [
                                        RustValue(
                                            EvalDeclare {
                                                name: "j",
                                                init_expr: Some(
                                                    RustValue(
                                                        EvalList {
                                                            elements: [
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: Int(
                                                                            10,
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: Int(
                                                                            20,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalFor {
                                                var_name: "i",
                                                iter_expr: RustValue(
                                                    EvalVariable {
                                                        name: "j",
                                                    },
                                                ),
                                                body: RustValue(
                                                    EvalBlock {
                                                        statements: [
                                                            RustValue(
                                                                EvalVariable {
                                                                    name: "i",
                                                                },
                                                            ),
                                                        ],
                                                        final_expr: None,
                                                    },
                                                ),
                                            },
                                        ),
                                    ],
                                    final_expr: Some(
                                        RustValue(
                                            EvalList {
                                                elements: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                2,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                3,
                                                            ),
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalVariable {
                                                            name: "f",
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                2,
                                                            ),
                                                        },
                                                    ),
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                3,
                                                            ),
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
            ],
        },
    ),
)
```