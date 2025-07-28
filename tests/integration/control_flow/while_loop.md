# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = 0;
var count = 0;
while x < 3 {
    count = count + 1;
    x = x + 1;
}
assert(x == 3);
assert(count == 3);
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
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(While),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Ident, "count"),
        Token(Equal),
        Token(Ident, "count"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "count"),
        Token(EqualEqual),
        Token(Int, "3"),
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
                    "count",
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
                While {
                    condition: Lt(
                        Identifier(
                            "x",
                        ),
                        Literal(
                            Int(
                                3,
                            ),
                        ),
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "count",
                                ),
                                op: Assign,
                                value: Add(
                                    Identifier(
                                        "count",
                                    ),
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ),
                            },
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
                                    3,
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
                                "count",
                            ),
                            Literal(
                                Int(
                                    3,
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
                            name: "x",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            0,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "count",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            0,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalWhile {
                            condition: EvalRef(
                                EvalCall {
                                    func_expr: EvalRef(
                                        EvalGetAttr {
                                            obj_expr: EvalRef(
                                                EvalVariable {
                                                    name: "x",
                                                },
                                            ),
                                            attr_name: "op_lt",
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            EvalLiteral {
                                                value: Int(
                                                    3,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                            body: EvalRef(
                                EvalBlock {
                                    statements: [
                                        EvalRef(
                                            EvalAssign {
                                                name: "count",
                                                expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalVariable {
                                                                        name: "count",
                                                                    },
                                                                ),
                                                                attr_name: "op_add",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            },
                                        ),
                                        EvalRef(
                                            EvalAssign {
                                                name: "x",
                                                expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalVariable {
                                                                        name: "x",
                                                                    },
                                                                ),
                                                                attr_name: "op_add",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            },
                                        ),
                                    ],
                                    final_expr: None,
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
                                                        name: "x",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
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
                                                        name: "count",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
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
    ),
)
```