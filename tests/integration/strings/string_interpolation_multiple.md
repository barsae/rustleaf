# Program
Status: 🟢
Assertions: 1

```rustleaf
var a = 3;
var b = 7;
var result = "${a} and ${b} equals ${a + b}";
assert(result == "3 and 7 equals 10");
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
        Token(Ident, "a"),
        Token(Equal),
        Token(Int, "3"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "b"),
        Token(Equal),
        Token(Int, "7"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(InterpolationStart),
        Token(Ident, "a"),
        Token(InterpolationEnd),
        Token(StringPart, " and "),
        Token(InterpolationStart),
        Token(Ident, "b"),
        Token(InterpolationEnd),
        Token(StringPart, " equals "),
        Token(InterpolationStart),
        Token(Ident, "a"),
        Token(Plus),
        Token(Ident, "b"),
        Token(InterpolationEnd),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "3 and 7 equals 10"),
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
                    "a",
                ),
                value: Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "b",
                ),
                value: Some(
                    Literal(
                        Int(
                            7,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Expression(
                                Identifier(
                                    "a",
                                ),
                            ),
                            Text(
                                " and ",
                            ),
                            Expression(
                                Identifier(
                                    "b",
                                ),
                            ),
                            Text(
                                " equals ",
                            ),
                            Expression(
                                Add(
                                    Identifier(
                                        "a",
                                    ),
                                    Identifier(
                                        "b",
                                    ),
                                ),
                            ),
                        ],
                    ),
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "3 and 7 equals 10",
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
            EvalProgram {
                statements: [
                    EvalRef(
                        EvalDeclare {
                            name: "a",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            3,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "b",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "result",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "str",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalVariable {
                                                                    name: "a",
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                                attr_name: "op_add",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: String(
                                                        " and ",
                                                    ),
                                                },
                                            ),
                                            EvalRef(
                                                EvalCall {
                                                    func_expr: EvalRef(
                                                        EvalVariable {
                                                            name: "str",
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            EvalVariable {
                                                                name: "b",
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            EvalRef(
                                                EvalLiteral {
                                                    value: String(
                                                        " equals ",
                                                    ),
                                                },
                                            ),
                                            EvalRef(
                                                EvalCall {
                                                    func_expr: EvalRef(
                                                        EvalVariable {
                                                            name: "str",
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            EvalCall {
                                                                func_expr: EvalRef(
                                                                    EvalGetAttr {
                                                                        obj_expr: EvalRef(
                                                                            EvalVariable {
                                                                                name: "a",
                                                                            },
                                                                        ),
                                                                        attr_name: "op_add",
                                                                    },
                                                                ),
                                                                args: [
                                                                    EvalRef(
                                                                        EvalVariable {
                                                                            name: "b",
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
                            ),
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalVariable {
                                                        name: "result",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: String(
                                                        "3 and 7 equals 10",
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
    ),
)
```