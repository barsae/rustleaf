# Program
Status: 🟢
Assertions: 1

```rustleaf
var s = 0;
for x in [1, 2, 3] {
    s += x;
}
assert(s == 6);
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
parse_expression: starting at position 5
parse_expression: starting at position 8
parse_expression: starting at position 9
parse_expression: success
parse_expression: starting at position 11
parse_expression: success
parse_expression: starting at position 13
parse_expression: success
parse_expression: success
parse_statement: starting at position 16
parse_expression: starting at position 18
parse_expression: success
parse_statement: parsed assignment
parse_expression: success
parse_statement: parsed block-like expression statement
parse_program: parsing statement at position 21
parse_statement: starting at position 21
parse_statement: falling back to expression statement
parse_expression: starting at position 21
parse_expression: starting at position 23
parse_expression: success
parse_expression: success
parse_program: parsed 3 statements
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
        Token(Ident, "s"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "x"),
        Token(In),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(LeftBrace),
        Token(Ident, "s"),
        Token(PlusEqual),
        Token(Ident, "x"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "s"),
        Token(EqualEqual),
        Token(Int, "6"),
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
                    "s",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "s",
                                ),
                                op: AddAssign,
                                value: Identifier(
                                    "x",
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                },
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "s",
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
                        name: "s",
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
                    EvalFor {
                        var_name: "x",
                        iter_expr: RustValue(
                            EvalList {
                                elements: [
                                    RustValue(
                                        EvalLiteral {
                                            value: Int(
                                                1,
                                            ),
                                        },
                                    ),
                                    RustValue(
                                        EvalLiteral {
                                            value: Int(
                                                2,
                                            ),
                                        },
                                    ),
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
                        body: RustValue(
                            EvalBlock {
                                statements: [
                                    RustValue(
                                        EvalAssign {
                                            name: "s",
                                            expr: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalVariable {
                                                                    name: "s",
                                                                },
                                                            ),
                                                            attr_name: "op_add",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalVariable {
                                                                name: "x",
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
                                                    name: "s",
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
            ],
        },
    ),
)
```