# Program
Status: 🟢
Assertions: 4

```rustleaf
var processor = |x| {
    var temp = x * 2;
    temp + 1
};

var complex_lambda = |y| {
    var first = y + 10;
    var second = first * 3;
    second - 5
};

assert(processor(5) == 11);
assert(processor(0) == 1);
assert(complex_lambda(2) == 31);
assert(complex_lambda(10) == 55);
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
        Token(Ident, "processor"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "temp"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Star),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Ident, "temp"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "complex_lambda"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "y"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "first"),
        Token(Equal),
        Token(Ident, "y"),
        Token(Plus),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "second"),
        Token(Equal),
        Token(Ident, "first"),
        Token(Star),
        Token(Int, "3"),
        Token(Semicolon),
        Token(Ident, "second"),
        Token(Minus),
        Token(Int, "5"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "processor"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "11"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "processor"),
        Token(LeftParen),
        Token(Int, "0"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "complex_lambda"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "31"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "complex_lambda"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "55"),
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
                    "processor",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "temp",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "x",
                                                ),
                                                Literal(
                                                    Int(
                                                        2,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Add(
                                        Identifier(
                                            "temp",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "complex_lambda",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "y",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "first",
                                        ),
                                        value: Some(
                                            Add(
                                                Identifier(
                                                    "y",
                                                ),
                                                Literal(
                                                    Int(
                                                        10,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    VarDecl {
                                        pattern: Variable(
                                            "second",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "first",
                                                ),
                                                Literal(
                                                    Int(
                                                        3,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Sub(
                                        Identifier(
                                            "second",
                                        ),
                                        Literal(
                                            Int(
                                                5,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
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
                            FunctionCall(
                                Identifier(
                                    "processor",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
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
                            FunctionCall(
                                Identifier(
                                    "processor",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ],
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "complex_lambda",
                                ),
                                [
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    31,
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
                            FunctionCall(
                                Identifier(
                                    "complex_lambda",
                                ),
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    55,
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
                                    name: "processor",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLambda {
                                                    data: LambdaData {
                                                        params: [
                                                            "x",
                                                        ],
                                                        body: Eval(
                                                            EvalRef(
                                                                RefCell {
                                                                    value: EvalBlock {
                                                                        statements: [
                                                                            EvalRef(
                                                                                RefCell {
                                                                                    value: EvalDeclare {
                                                                                        name: "temp",
                                                                                        init_expr: Some(
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
                                                                                                                    attr_name: "op_mul",
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
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
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
                                                                                                                name: "temp",
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
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
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
                                    name: "complex_lambda",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLambda {
                                                    data: LambdaData {
                                                        params: [
                                                            "y",
                                                        ],
                                                        body: Eval(
                                                            EvalRef(
                                                                RefCell {
                                                                    value: EvalBlock {
                                                                        statements: [
                                                                            EvalRef(
                                                                                RefCell {
                                                                                    value: EvalDeclare {
                                                                                        name: "first",
                                                                                        init_expr: Some(
                                                                                            EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalCall {
                                                                                                        func_expr: EvalRef(
                                                                                                            RefCell {
                                                                                                                value: EvalGetAttr {
                                                                                                                    obj_expr: EvalRef(
                                                                                                                        RefCell {
                                                                                                                            value: EvalVariable {
                                                                                                                                name: "y",
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
                                                                                                                            10,
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
                                                                            EvalRef(
                                                                                RefCell {
                                                                                    value: EvalDeclare {
                                                                                        name: "second",
                                                                                        init_expr: Some(
                                                                                            EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalCall {
                                                                                                        func_expr: EvalRef(
                                                                                                            RefCell {
                                                                                                                value: EvalGetAttr {
                                                                                                                    obj_expr: EvalRef(
                                                                                                                        RefCell {
                                                                                                                            value: EvalVariable {
                                                                                                                                name: "first",
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
                                                                                                                            3,
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
                                                                                                                name: "second",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    attr_name: "op_sub",
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
                                                        ),
                                                    },
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
                                                                        value: EvalCall {
                                                                            func_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalVariable {
                                                                                        name: "processor",
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
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        11,
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
                                                                        value: EvalCall {
                                                                            func_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalVariable {
                                                                                        name: "processor",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            args: [
                                                                                EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                0,
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
                                                                                        name: "complex_lambda",
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
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        31,
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
                                                                        value: EvalCall {
                                                                            func_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalVariable {
                                                                                        name: "complex_lambda",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            args: [
                                                                                EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                10,
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
                                                                        55,
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