# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 0;
loop {
    x += 1;
    break;
    x += 1;
}
assert(x == 1);
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
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Break),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
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
                            Break(
                                None,
                            ),
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
    Eval(
        EvalRef(
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "x",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: Int(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalLoop {
                                    body: EvalRef(
                                        RefCell {
                                            value: EvalBlock {
                                                statements: [
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalAssign {
                                                                name: "x",
                                                                expr: EvalRef(
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
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                1,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalBreak {
                                                                expr: None,
                                                            },
                                                        },
                                                    ),
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalAssign {
                                                                name: "x",
                                                                expr: EvalRef(
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
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                1,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_expr: None,
                                            },
                                        },
                                    ),
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
                                                                        value: EvalVariable {
                                                                            name: "x",
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
                                                                        1,
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