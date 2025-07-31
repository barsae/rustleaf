# Program
Status: 🔴
Assertions: 0

```rustleaf
// Test is_unit with various values
assert(not is_unit(42));
assert(not is_unit("hello"));
assert(not is_unit(true));
assert(not is_unit([]));
assert(not is_unit({}));

// Test with a function that returns Unit
fn side_effect() {
    var x = 1;
}
assert(is_unit(side_effect()));
```

# Output
None

# Result
```rust
Err(
    "Value is not callable: Bool(false)",
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(LeftBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "side_effect"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "side_effect"),
        Token(LeftParen),
        Token(RightParen),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        FunctionCall(
                            Not(
                                Identifier(
                                    "is_unit",
                                ),
                            ),
                            [
                                Literal(
                                    Int(
                                        42,
                                    ),
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
                        FunctionCall(
                            Not(
                                Identifier(
                                    "is_unit",
                                ),
                            ),
                            [
                                Literal(
                                    String(
                                        "hello",
                                    ),
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
                        FunctionCall(
                            Not(
                                Identifier(
                                    "is_unit",
                                ),
                            ),
                            [
                                Literal(
                                    Bool(
                                        true,
                                    ),
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
                        FunctionCall(
                            Not(
                                Identifier(
                                    "is_unit",
                                ),
                            ),
                            [
                                List(
                                    [],
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
                        FunctionCall(
                            Not(
                                Identifier(
                                    "is_unit",
                                ),
                            ),
                            [
                                Dict(
                                    [],
                                ),
                            ],
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "side_effect",
                params: [],
                body: Block {
                    statements: [
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
                                        "side_effect",
                                    ),
                                    [],
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
    RustValue(
        EvalProgram {
            statements: [
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
                                        EvalLogicalNot {
                                            expr: RustValue(
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                        EvalLogicalNot {
                                            expr: RustValue(
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "hello",
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                        EvalLogicalNot {
                                            expr: RustValue(
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Bool(
                                                    true,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                        EvalLogicalNot {
                                            expr: RustValue(
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                        EvalLogicalNot {
                                            expr: RustValue(
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalDict {
                                                pairs: [],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "side_effect",
                            params: [],
                            body: RustValue(
                                EvalBlock {
                                    statements: [
                                        RustValue(
                                            EvalDeclare {
                                                name: "x",
                                                init_expr: Some(
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ],
                                    final_expr: None,
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
                                        EvalVariable {
                                            name: "is_unit",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "side_effect",
                                                    },
                                                ),
                                                args: [],
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