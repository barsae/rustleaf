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
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_expression: starting at position 3
parse_statement: starting at position 5
parse_expression: starting at position 6
parse_expression: success
parse_statement: parsed break statement
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 10
parse_statement: starting at position 10
parse_expression: starting at position 13
parse_statement: starting at position 15
parse_expression: starting at position 16
parse_expression: success
parse_statement: parsed break statement
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 20
parse_statement: starting at position 20
parse_expression: starting at position 23
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 25
parse_statement: starting at position 25
parse_expression: starting at position 28
parse_statement: starting at position 30
parse_expression: starting at position 32
parse_expression: success
parse_statement: parsed assignment
parse_statement: starting at position 36
parse_expression: starting at position 36
parse_expression: starting at position 37
parse_expression: success
parse_statement: starting at position 41
parse_expression: starting at position 42
parse_expression: success
parse_statement: parsed break statement
parse_expression: success
parse_statement: parsed block-like expression statement
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 49
parse_statement: starting at position 49
parse_statement: falling back to expression statement
parse_expression: starting at position 49
parse_expression: starting at position 51
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 56
parse_statement: starting at position 56
parse_statement: falling back to expression statement
parse_expression: starting at position 56
parse_expression: starting at position 58
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 63
parse_statement: starting at position 63
parse_statement: falling back to expression statement
parse_expression: starting at position 63
parse_expression: starting at position 65
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 70
parse_statement: starting at position 70
parse_statement: falling back to expression statement
parse_expression: starting at position 70
parse_expression: starting at position 72
parse_expression: success
parse_expression: success
parse_program: parsed 8 statements
```

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
                                Expression(
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
                            ],
                            final_expr: None,
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalDeclare {
                        name: "result",
                        init_expr: Some(
                            RustValue(
                                EvalLoop {
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalBreak {
                                                        expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        42,
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
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result2",
                        init_expr: Some(
                            RustValue(
                                EvalLoop {
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalBreak {
                                                        expr: Some(
                                                            RustValue(
                                                                EvalLiteral {
                                                                    value: String(
                                                                        "hello",
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
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "counter",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result3",
                        init_expr: Some(
                            RustValue(
                                EvalLoop {
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalAssign {
                                                        name: "counter",
                                                        expr: RustValue(
                                                            EvalCall {
                                                                func_expr: RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "counter",
                                                                            },
                                                                        ),
                                                                        attr_name: "op_add",
                                                                    },
                                                                ),
                                                                args: [
                                                                    RustValue(
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
                                                RustValue(
                                                    EvalIf {
                                                        condition: RustValue(
                                                            EvalCall {
                                                                func_expr: RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "counter",
                                                                            },
                                                                        ),
                                                                        attr_name: "op_eq",
                                                                    },
                                                                ),
                                                                args: [
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
                                                        then_expr: RustValue(
                                                            EvalBlock {
                                                                statements: [
                                                                    RustValue(
                                                                        EvalBreak {
                                                                            expr: Some(
                                                                                RustValue(
                                                                                    EvalCall {
                                                                                        func_expr: RustValue(
                                                                                            EvalGetAttr {
                                                                                                obj_expr: RustValue(
                                                                                                    EvalVariable {
                                                                                                        name: "counter",
                                                                                                    },
                                                                                                ),
                                                                                                attr_name: "op_mul",
                                                                                            },
                                                                                        ),
                                                                                        args: [
                                                                                            RustValue(
                                                                                                EvalLiteral {
                                                                                                    value: Int(
                                                                                                        5,
                                                                                                    ),
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ],
                                                                final_expr: None,
                                                            },
                                                        ),
                                                        else_expr: None,
                                                    },
                                                ),
                                            ],
                                            final_expr: None,
                                        },
                                    ),
                                },
                            ),
                        ),
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
                                                EvalVariable {
                                                    name: "result",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "result2",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "result3",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    15,
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
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "counter",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
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
)
```