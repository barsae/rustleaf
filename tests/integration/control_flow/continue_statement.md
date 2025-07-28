# Program
Status: 🟢
Assertions: 1

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
                                                        1,
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
                                                            value: EvalIf {
                                                                condition: EvalRef(
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
                                                                                        attr_name: "op_lt",
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
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                                then_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalBlock {
                                                                            statements: [
                                                                                EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalContinue,
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_expr: None,
                                                                        },
                                                                    },
                                                                ),
                                                                else_expr: None,
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
                                                                        2,
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