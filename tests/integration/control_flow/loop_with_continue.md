# Program
Status: 🟢
Assertions: 5

```rustleaf
var i = 0;
var result = loop {
    i = i + 1;
    if i < 3 {
        continue;
    }
    break i * 10;
};

var j = 0;
var count = 0;
var result2 = loop {
    j = j + 1;
    if j <= 5 {
        count = count + 1;
        continue;
    }
    break j + count;
};

assert(result == 30);
assert(i == 3);
assert(result2 == 11);  
assert(j == 6);
assert(count == 5);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_expression: starting at position 3
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 5
parse_statement: starting at position 5
parse_expression: starting at position 8
parse_statement: starting at position 10
parse_expression: starting at position 12
parse_expression: success
parse_statement: parsed assignment
parse_statement: starting at position 16
parse_expression: starting at position 16
parse_expression: starting at position 17
parse_expression: success
parse_statement: starting at position 21
parse_statement: parsed continue statement
parse_expression: success
parse_statement: parsed block-like expression statement
parse_statement: starting at position 24
parse_expression: starting at position 25
parse_expression: success
parse_statement: parsed break statement
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 31
parse_statement: starting at position 31
parse_expression: starting at position 34
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 36
parse_statement: starting at position 36
parse_expression: starting at position 39
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 41
parse_statement: starting at position 41
parse_expression: starting at position 44
parse_statement: starting at position 46
parse_expression: starting at position 48
parse_expression: success
parse_statement: parsed assignment
parse_statement: starting at position 52
parse_expression: starting at position 52
parse_expression: starting at position 53
parse_expression: success
parse_statement: starting at position 57
parse_expression: starting at position 59
parse_expression: success
parse_statement: parsed assignment
parse_statement: starting at position 63
parse_statement: parsed continue statement
parse_expression: success
parse_statement: parsed block-like expression statement
parse_statement: starting at position 66
parse_expression: starting at position 67
parse_expression: success
parse_statement: parsed break statement
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 73
parse_statement: starting at position 73
parse_statement: falling back to expression statement
parse_expression: starting at position 73
parse_expression: starting at position 75
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 80
parse_statement: starting at position 80
parse_statement: falling back to expression statement
parse_expression: starting at position 80
parse_expression: starting at position 82
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 87
parse_statement: starting at position 87
parse_statement: falling back to expression statement
parse_expression: starting at position 87
parse_expression: starting at position 89
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 94
parse_statement: starting at position 94
parse_statement: falling back to expression statement
parse_expression: starting at position 94
parse_expression: starting at position 96
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 101
parse_statement: starting at position 101
parse_statement: falling back to expression statement
parse_expression: starting at position 101
parse_expression: starting at position 103
parse_expression: success
parse_expression: success
parse_program: parsed 10 statements
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
        Token(Ident, "i"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Equal),
        Token(Ident, "i"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "i"),
        Token(Less),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Ident, "i"),
        Token(Star),
        Token(Int, "10"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "j"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "j"),
        Token(Equal),
        Token(Ident, "j"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "j"),
        Token(LessEqual),
        Token(Int, "5"),
        Token(LeftBrace),
        Token(Ident, "count"),
        Token(Equal),
        Token(Ident, "count"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Ident, "j"),
        Token(Plus),
        Token(Ident, "count"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "30"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "i"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Int, "11"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "j"),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "count"),
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
            VarDecl {
                pattern: Variable(
                    "i",
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
                    "result",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "i",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "i",
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
                                        condition: Lt(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    3,
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
                                    Some(
                                        Mul(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    10,
                                                ),
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
                    "j",
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
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "j",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "j",
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
                                        condition: Le(
                                            Identifier(
                                                "j",
                                            ),
                                            Literal(
                                                Int(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
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
                                                Continue,
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                                Break(
                                    Some(
                                        Add(
                                            Identifier(
                                                "j",
                                            ),
                                            Identifier(
                                                "count",
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
                                    30,
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
                                "i",
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
                                "result2",
                            ),
                            Literal(
                                Int(
                                    11,
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
                                "j",
                            ),
                            Literal(
                                Int(
                                    6,
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalDeclare {
                        name: "i",
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
                        name: "result",
                        init_expr: Some(
                            RustValue(
                                EvalLoop {
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalAssign {
                                                        name: "i",
                                                        expr: RustValue(
                                                            EvalCall {
                                                                func_expr: RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "i",
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
                                                                                name: "i",
                                                                            },
                                                                        ),
                                                                        attr_name: "op_lt",
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
                                                                        EvalContinue,
                                                                    ),
                                                                ],
                                                                final_expr: None,
                                                            },
                                                        ),
                                                        else_expr: None,
                                                    },
                                                ),
                                                RustValue(
                                                    EvalBreak {
                                                        expr: Some(
                                                            RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "i",
                                                                                },
                                                                            ),
                                                                            attr_name: "op_mul",
                                                                        },
                                                                    ),
                                                                    args: [
                                                                        RustValue(
                                                                            EvalLiteral {
                                                                                value: Int(
                                                                                    10,
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
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "j",
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
                        name: "count",
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
                        name: "result2",
                        init_expr: Some(
                            RustValue(
                                EvalLoop {
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalAssign {
                                                        name: "j",
                                                        expr: RustValue(
                                                            EvalCall {
                                                                func_expr: RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "j",
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
                                                                                name: "j",
                                                                            },
                                                                        ),
                                                                        attr_name: "op_le",
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
                                                        then_expr: RustValue(
                                                            EvalBlock {
                                                                statements: [
                                                                    RustValue(
                                                                        EvalAssign {
                                                                            name: "count",
                                                                            expr: RustValue(
                                                                                EvalCall {
                                                                                    func_expr: RustValue(
                                                                                        EvalGetAttr {
                                                                                            obj_expr: RustValue(
                                                                                                EvalVariable {
                                                                                                    name: "count",
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
                                                                        EvalContinue,
                                                                    ),
                                                                ],
                                                                final_expr: None,
                                                            },
                                                        ),
                                                        else_expr: None,
                                                    },
                                                ),
                                                RustValue(
                                                    EvalBreak {
                                                        expr: Some(
                                                            RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "j",
                                                                                },
                                                                            ),
                                                                            attr_name: "op_add",
                                                                        },
                                                                    ),
                                                                    args: [
                                                                        RustValue(
                                                                            EvalVariable {
                                                                                name: "count",
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
                                                    30,
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
                                                    name: "i",
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
                                                value: Int(
                                                    11,
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
                                                    name: "j",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    6,
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
                                                    name: "count",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                        ],
                    },
                ),
            ],
        },
    ),
)
```