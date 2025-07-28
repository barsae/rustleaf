# Program
Status: 🟢
Assertions: 4

```rustleaf
var x = 1;
var result = if x > 0 {
    "positive"
} else {
    "zero or negative"
};

var y = -5;
var result2 = if y > 0 {
    "positive"
} else {
    "zero or negative"
};

assert(result == "positive");
assert(result2 == "zero or negative");
assert(x == 1);
assert(y == -5);
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
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "positive"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "zero or negative"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "y"),
        Token(Equal),
        Token(Minus),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(If),
        Token(Ident, "y"),
        Token(Greater),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "positive"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "zero or negative"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "positive"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(String, "zero or negative"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(EqualEqual),
        Token(Minus),
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
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        String(
                                            "zero or negative",
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
                    "y",
                ),
                value: Some(
                    Neg(
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "y",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        String(
                                            "zero or negative",
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
                            Identifier(
                                "result",
                            ),
                            Literal(
                                String(
                                    "positive",
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
                                    "zero or negative",
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
                            Neg(
                                Literal(
                                    Int(
                                        5,
                                    ),
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
                name: "x",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            1,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result",
                init_expr: Some(
                    EvalIf {
                        condition: EvalCall {
                            func_expr: EvalGetAttr {
                                obj_expr: EvalVariable {
                                    name: "x",
                                },
                                attr_name: "op_gt",
                            },
                            args: [
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ],
                        },
                        then_expr: EvalBlock {
                            statements: [],
                            final_expr: Some(
                                EvalLiteral {
                                    value: String(
                                        "positive",
                                    ),
                                },
                            ),
                        },
                        else_expr: Some(
                            EvalBlock {
                                statements: [],
                                final_expr: Some(
                                    EvalLiteral {
                                        value: String(
                                            "zero or negative",
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "y",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: Int(
                                    5,
                                ),
                            },
                            attr_name: "op_neg",
                        },
                        args: [],
                    },
                ),
            },
            EvalDeclare {
                name: "result2",
                init_expr: Some(
                    EvalIf {
                        condition: EvalCall {
                            func_expr: EvalGetAttr {
                                obj_expr: EvalVariable {
                                    name: "y",
                                },
                                attr_name: "op_gt",
                            },
                            args: [
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ],
                        },
                        then_expr: EvalBlock {
                            statements: [],
                            final_expr: Some(
                                EvalLiteral {
                                    value: String(
                                        "positive",
                                    ),
                                },
                            ),
                        },
                        else_expr: Some(
                            EvalBlock {
                                statements: [],
                                final_expr: Some(
                                    EvalLiteral {
                                        value: String(
                                            "zero or negative",
                                        ),
                                    },
                                ),
                            },
                        ),
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
                                value: String(
                                    "positive",
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
                                value: String(
                                    "zero or negative",
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
                                name: "x",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    1,
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
                                name: "y",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalCall {
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                    attr_name: "op_neg",
                                },
                                args: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
)
```