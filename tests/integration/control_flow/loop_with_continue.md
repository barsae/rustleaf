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
    EvalProgram {
        statements: [
            EvalDeclare {
                name: "i",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            0,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result",
                init_expr: Some(
                    EvalLoop {
                        body: EvalBlock {
                            statements: [
                                EvalAssign {
                                    name: "i",
                                    expr: EvalCall {
                                        func_expr: EvalGetAttr {
                                            obj_expr: EvalVariable {
                                                name: "i",
                                            },
                                            attr_name: "op_add",
                                        },
                                        args: [
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ],
                                    },
                                },
                                EvalIf {
                                    condition: EvalCall {
                                        func_expr: EvalGetAttr {
                                            obj_expr: EvalVariable {
                                                name: "i",
                                            },
                                            attr_name: "op_lt",
                                        },
                                        args: [
                                            EvalLiteral {
                                                value: Int(
                                                    3,
                                                ),
                                            },
                                        ],
                                    },
                                    then_expr: EvalBlock {
                                        statements: [
                                            EvalContinue,
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: None,
                                },
                                EvalBreak {
                                    expr: Some(
                                        EvalCall {
                                            func_expr: EvalGetAttr {
                                                obj_expr: EvalVariable {
                                                    name: "i",
                                                },
                                                attr_name: "op_mul",
                                            },
                                            args: [
                                                EvalLiteral {
                                                    value: Int(
                                                        10,
                                                    ),
                                                },
                                            ],
                                        },
                                    ),
                                },
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            EvalDeclare {
                name: "j",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            0,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "count",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            0,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result2",
                init_expr: Some(
                    EvalLoop {
                        body: EvalBlock {
                            statements: [
                                EvalAssign {
                                    name: "j",
                                    expr: EvalCall {
                                        func_expr: EvalGetAttr {
                                            obj_expr: EvalVariable {
                                                name: "j",
                                            },
                                            attr_name: "op_add",
                                        },
                                        args: [
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ],
                                    },
                                },
                                EvalIf {
                                    condition: EvalCall {
                                        func_expr: EvalGetAttr {
                                            obj_expr: EvalVariable {
                                                name: "j",
                                            },
                                            attr_name: "op_le",
                                        },
                                        args: [
                                            EvalLiteral {
                                                value: Int(
                                                    5,
                                                ),
                                            },
                                        ],
                                    },
                                    then_expr: EvalBlock {
                                        statements: [
                                            EvalAssign {
                                                name: "count",
                                                expr: EvalCall {
                                                    func_expr: EvalGetAttr {
                                                        obj_expr: EvalVariable {
                                                            name: "count",
                                                        },
                                                        attr_name: "op_add",
                                                    },
                                                    args: [
                                                        EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    ],
                                                },
                                            },
                                            EvalContinue,
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: None,
                                },
                                EvalBreak {
                                    expr: Some(
                                        EvalCall {
                                            func_expr: EvalGetAttr {
                                                obj_expr: EvalVariable {
                                                    name: "j",
                                                },
                                                attr_name: "op_add",
                                            },
                                            args: [
                                                EvalVariable {
                                                    name: "count",
                                                },
                                            ],
                                        },
                                    ),
                                },
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "result",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    30,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "i",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    3,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "result2",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    11,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "j",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    6,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "count",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    5,
                                ),
                            },
                        ],
                    },
                ],
            },
        ],
    },
)
```