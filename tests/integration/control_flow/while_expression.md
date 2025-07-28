# Program
Status: 🟢
Assertions: 2

```rustleaf
// Test while loop as expression with proper variable initialization
var x = 0;
var result = while x < 5 {
    x = x + 1;
};
assert(x == 5);
assert(is_unit(result));
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
        Token(Ident, "result"),
        Token(Equal),
        Token(While),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "5"),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(RightParen),
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
                    "result",
                ),
                value: Some(
                    While {
                        condition: Lt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ),
                        body: Block {
                            statements: [
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
            },
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
                                    5,
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
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
                                Identifier(
                                    "result",
                                ),
                            ],
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
                            0,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result",
                init_expr: Some(
                    EvalWhile {
                        condition: EvalCall {
                            func_expr: EvalGetAttr {
                                obj_expr: EvalVariable {
                                    name: "x",
                                },
                                attr_name: "op_lt",
                            },
                            args: [
                                EvalLiteral {
                                    value: Int(
                                        5,
                                    ),
                                },
                            ],
                        },
                        body: EvalBlock {
                            statements: [
                                EvalAssign {
                                    name: "x",
                                    expr: EvalCall {
                                        func_expr: EvalGetAttr {
                                            obj_expr: EvalVariable {
                                                name: "x",
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
                                name: "x",
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalVariable {
                            name: "is_unit",
                        },
                        args: [
                            EvalVariable {
                                name: "result",
                            },
                        ],
                    },
                ],
            },
        ],
    },
)
```