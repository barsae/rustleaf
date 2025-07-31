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
parse_expression: starting at position 9
parse_expression: success
parse_statement: starting at position 13
parse_expression: starting at position 15
parse_expression: success
parse_statement: parsed assignment
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 21
parse_statement: starting at position 21
parse_statement: falling back to expression statement
parse_expression: starting at position 21
parse_expression: starting at position 23
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 28
parse_statement: starting at position 28
parse_statement: falling back to expression statement
parse_expression: starting at position 28
parse_expression: starting at position 30
parse_expression: starting at position 32
parse_expression: success
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
                                EvalWhile {
                                    condition: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "x",
                                                        },
                                                    ),
                                                    attr_name: "op_lt",
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
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalAssign {
                                                        name: "x",
                                                        expr: RustValue(
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
                                                    name: "x",
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
                                        EvalVariable {
                                            name: "is_unit",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalVariable {
                                                name: "result",
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