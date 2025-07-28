# Program
Status: 🟢
Assertions: 3

```rustleaf
var result1 = if true { 42 } else { 0 };
var result2 = if false { 100 } else { 200 };
var x = 5;
var result3 = if x > 3 { "big" } else { "small" };
assert(result1 == 42);
assert(result2 == 200);
assert(result3 == "big");
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
        Token(Ident, "result1"),
        Token(Equal),
        Token(If),
        Token(True),
        Token(LeftBrace),
        Token(Int, "42"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(If),
        Token(False),
        Token(LeftBrace),
        Token(Int, "100"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Int, "200"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(String, "big"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "small"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result1"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Int, "200"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(String, "big"),
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
                    "result1",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                true,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            0,
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
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                false,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            200,
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "big",
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
                                            "small",
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
                                "result1",
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
                                Int(
                                    200,
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
                                String(
                                    "big",
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
                name: "result1",
                init_expr: Some(
                    EvalIf {
                        condition: EvalLiteral {
                            value: Bool(
                                true,
                            ),
                        },
                        then_expr: EvalBlock {
                            statements: [],
                            final_expr: Some(
                                EvalLiteral {
                                    value: Int(
                                        42,
                                    ),
                                },
                            ),
                        },
                        else_expr: Some(
                            EvalBlock {
                                statements: [],
                                final_expr: Some(
                                    EvalLiteral {
                                        value: Int(
                                            0,
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result2",
                init_expr: Some(
                    EvalIf {
                        condition: EvalLiteral {
                            value: Bool(
                                false,
                            ),
                        },
                        then_expr: EvalBlock {
                            statements: [],
                            final_expr: Some(
                                EvalLiteral {
                                    value: Int(
                                        100,
                                    ),
                                },
                            ),
                        },
                        else_expr: Some(
                            EvalBlock {
                                statements: [],
                                final_expr: Some(
                                    EvalLiteral {
                                        value: Int(
                                            200,
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "x",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            5,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result3",
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
                                        3,
                                    ),
                                },
                            ],
                        },
                        then_expr: EvalBlock {
                            statements: [],
                            final_expr: Some(
                                EvalLiteral {
                                    value: String(
                                        "big",
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
                                            "small",
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
                                name: "result1",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    42,
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
                                    200,
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
                                name: "result3",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "big",
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