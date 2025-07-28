# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 10;
var y = 5;
var result = "Value: ${x + y * 2}";
assert(result == "Value: 20");
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
        Token(Int, "10"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(StringPart, "Value: "),
        Token(InterpolationStart),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(Star),
        Token(Int, "2"),
        Token(InterpolationEnd),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "Value: 20"),
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
                            10,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "y",
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
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Text(
                                "Value: ",
                            ),
                            Expression(
                                Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Mul(
                                        Identifier(
                                            "y",
                                        ),
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ],
                    ),
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
                                    "Value: 20",
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
                            10,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "y",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            5,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: String(
                                    "Value: ",
                                ),
                            },
                            attr_name: "op_add",
                        },
                        args: [
                            EvalCall {
                                func_expr: EvalVariable {
                                    name: "str",
                                },
                                args: [
                                    EvalCall {
                                        func_expr: EvalGetAttr {
                                            obj_expr: EvalVariable {
                                                name: "x",
                                            },
                                            attr_name: "op_add",
                                        },
                                        args: [
                                            EvalCall {
                                                func_expr: EvalGetAttr {
                                                    obj_expr: EvalVariable {
                                                        name: "y",
                                                    },
                                                    attr_name: "op_mul",
                                                },
                                                args: [
                                                    EvalLiteral {
                                                        value: Int(
                                                            2,
                                                        ),
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
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
                                    "Value: 20",
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