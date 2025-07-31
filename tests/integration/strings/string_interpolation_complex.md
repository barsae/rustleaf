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
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 10
parse_statement: starting at position 10
parse_expression: starting at position 13
parse_expression: starting at position 15
parse_expression: success
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 22
parse_statement: starting at position 22
parse_statement: falling back to expression statement
parse_expression: starting at position 22
parse_expression: starting at position 24
parse_expression: success
parse_expression: success
parse_program: parsed 4 statements
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalDeclare {
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        10,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "y",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        5,
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
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalLiteral {
                                                    value: String(
                                                        "Value: ",
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_add",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "str",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalCall {
                                                            func_expr: RustValue(
                                                                EvalGetAttr {
                                                                    obj_expr: RustValue(
                                                                        EvalVariable {
                                                                            name: "x",
                                                                        },
                                                                    ),
                                                                    attr_name: "op_add",
                                                                },
                                                            ),
                                                            args: [
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalGetAttr {
                                                                                obj_expr: RustValue(
                                                                                    EvalVariable {
                                                                                        name: "y",
                                                                                    },
                                                                                ),
                                                                                attr_name: "op_mul",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalLiteral {
                                                                                    value: Int(
                                                                                        2,
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
                                    ],
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
                                                value: String(
                                                    "Value: 20",
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