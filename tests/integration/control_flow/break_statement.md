# Program
Status: 🟢
Assertions: 4

```rustleaf
var result = loop {
    break 42;
};

var result2 = loop {
    break "hello";
};

var counter = 0;
var result3 = loop {
    counter = counter + 1;
    if counter == 3 {
        break counter * 5;
    }
};

assert(result == 42);
assert(result2 == "hello");
assert(result3 == 15);
assert(counter == 3);
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
        Token(Ident, "result"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(String, "hello"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Ident, "counter"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "counter"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Break),
        Token(Ident, "counter"),
        Token(Star),
        Token(Int, "5"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(String, "hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "counter"),
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
                    "result",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Break(
                                    Some(
                                        Literal(
                                            Int(
                                                42,
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Break(
                                    Some(
                                        Literal(
                                            String(
                                                "hello",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "counter",
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
                    "result3",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "counter",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "counter",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                            ],
                            final_expr: Some(
                                If {
                                    condition: Eq(
                                        Identifier(
                                            "counter",
                                        ),
                                        Literal(
                                            Int(
                                                3,
                                            ),
                                        ),
                                    ),
                                    then_expr: Block {
                                        statements: [
                                            Break(
                                                Some(
                                                    Mul(
                                                        Identifier(
                                                            "counter",
                                                        ),
                                                        Literal(
                                                            Int(
                                                                5,
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: None,
                                },
                            ),
                        },
                    },
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
                                Int(
                                    42,
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
                                "result2",
                            ),
                            Literal(
                                String(
                                    "hello",
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
                                "result3",
                            ),
                            Literal(
                                Int(
                                    15,
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
                                "counter",
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
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "result",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLoop {
                                                    body: EvalRef(
                                                        RefCell {
                                                            value: EvalBlock {
                                                                statements: [
                                                                    EvalRef(
                                                                        RefCell {
                                                                            value: EvalBreak {
                                                                                expr: Some(
                                                                                    EvalRef(
                                                                                        RefCell {
                                                                                            value: EvalLiteral {
                                                                                                value: Int(
                                                                                                    42,
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    ),
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
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "result2",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLoop {
                                                    body: EvalRef(
                                                        RefCell {
                                                            value: EvalBlock {
                                                                statements: [
                                                                    EvalRef(
                                                                        RefCell {
                                                                            value: EvalBreak {
                                                                                expr: Some(
                                                                                    EvalRef(
                                                                                        RefCell {
                                                                                            value: EvalLiteral {
                                                                                                value: String(
                                                                                                    "hello",
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    ),
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
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "counter",
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
                                value: EvalDeclare {
                                    name: "result3",
                                    init_expr: Some(
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
                                                                                name: "counter",
                                                                                expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalCall {
                                                                                            func_expr: EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalGetAttr {
                                                                                                        obj_expr: EvalRef(
                                                                                                            RefCell {
                                                                                                                value: EvalVariable {
                                                                                                                    name: "counter",
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
                                                                final_expr: Some(
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
                                                                                                                    name: "counter",
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
                                                                                                                3,
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
                                                                                                        value: EvalBreak {
                                                                                                            expr: Some(
                                                                                                                EvalRef(
                                                                                                                    RefCell {
                                                                                                                        value: EvalCall {
                                                                                                                            func_expr: EvalRef(
                                                                                                                                RefCell {
                                                                                                                                    value: EvalGetAttr {
                                                                                                                                        obj_expr: EvalRef(
                                                                                                                                            RefCell {
                                                                                                                                                value: EvalVariable {
                                                                                                                                                    name: "counter",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        attr_name: "op_mul",
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
                                                                                                            ),
                                                                                                        },
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
                                                                ),
                                                            },
                                                        },
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
                                                                            name: "result",
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
                                                                        42,
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
                                                                            name: "result2",
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
                                                                    value: String(
                                                                        "hello",
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
                                                                            name: "result3",
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
                                                                        15,
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
                                                                            name: "counter",
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
                                                                        3,
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