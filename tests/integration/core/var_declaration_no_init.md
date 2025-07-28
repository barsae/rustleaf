# Program
Status: 🟢
Assertions: 3

```rustleaf
var y;
var z;
y = 100;
z = "assigned later";
assert(y == 100);
assert(z == "assigned later");
assert(y + 23 == 123);
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
        Token(Ident, "y"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "z"),
        Token(Semicolon),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "100"),
        Token(Semicolon),
        Token(Ident, "z"),
        Token(Equal),
        Token(String, "assigned later"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(EqualEqual),
        Token(Int, "100"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
        Token(EqualEqual),
        Token(String, "assigned later"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(Plus),
        Token(Int, "23"),
        Token(EqualEqual),
        Token(Int, "123"),
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
                    "y",
                ),
                value: None,
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: None,
            },
            Assignment {
                target: Identifier(
                    "y",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        100,
                    ),
                ),
            },
            Assignment {
                target: Identifier(
                    "z",
                ),
                op: Assign,
                value: Literal(
                    String(
                        "assigned later",
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
                                "y",
                            ),
                            Literal(
                                Int(
                                    100,
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
                        Eq(
                            Identifier(
                                "z",
                            ),
                            Literal(
                                String(
                                    "assigned later",
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
                        Eq(
                            Add(
                                Identifier(
                                    "y",
                                ),
                                Literal(
                                    Int(
                                        23,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    123,
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
                            name: "y",
                            init_expr: None,
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "z",
                            init_expr: None,
                        },
                    ),
                    EvalRef(
                        EvalAssign {
                            name: "y",
                            expr: EvalRef(
                                EvalLiteral {
                                    value: Int(
                                        100,
                                    ),
                                },
                            ),
                        },
                    ),
                    EvalRef(
                        EvalAssign {
                            name: "z",
                            expr: EvalRef(
                                EvalLiteral {
                                    value: String(
                                        "assigned later",
                                    ),
                                },
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
                                                        name: "y",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        100,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
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
                                                        name: "z",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: String(
                                                        "assigned later",
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
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
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalVariable {
                                                                        name: "y",
                                                                    },
                                                                ),
                                                                attr_name: "op_add",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        23,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        123,
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