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
        Token(Fn),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "5"),
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
    Eval(
        EvalRef(
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalFunction {
                                    data: FunctionData {
                                        name: "add",
                                        params: [
                                            (
                                                "x",
                                                None,
                                                Regular,
                                            ),
                                            (
                                                "y",
                                                None,
                                                Regular,
                                            ),
                                        ],
                                        body: Eval(
                                            EvalRef(
                                                RefCell {
                                                    value: EvalBlock {
                                                        statements: [],
                                                        final_expr: Some(
                                                            EvalRef(
                                                                RefCell {
                                                                    value: EvalCall {
                                                                        func_expr: EvalRef(
                                                                            RefCell {
                                                                                value: EvalGetAttr {
                                                                                    obj_expr: EvalRef(
                                                                                        RefCell {
                                                                                            value: EvalVariable {
                                                                                                name: "x",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "op_add",
                                                                                },
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            EvalRef(
                                                                                RefCell {
                                                                                    value: EvalVariable {
                                                                                        name: "y",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalCall {
                                                                            func_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalVariable {
                                                                                        name: "add",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            args: [
                                                                                EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                2,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                3,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        5,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                    ],
                },
            },
        ),
    ),
)
```