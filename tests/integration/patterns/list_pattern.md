# Program
Status: 🟢
Assertions: 3

```rustleaf
var [a, b, c] = [1, 2, 3];
assert(a == 1);
assert(b == 2);
assert(c == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_expression: starting at position 9
parse_expression: starting at position 10
parse_expression: success
parse_expression: starting at position 12
parse_expression: success
parse_expression: starting at position 14
parse_expression: success
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 17
parse_statement: starting at position 17
parse_statement: falling back to expression statement
parse_expression: starting at position 17
parse_expression: starting at position 19
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 24
parse_statement: starting at position 24
parse_statement: falling back to expression statement
parse_expression: starting at position 24
parse_expression: starting at position 26
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 31
parse_statement: starting at position 31
parse_statement: falling back to expression statement
parse_expression: starting at position 31
parse_expression: starting at position 33
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
        Token(LeftBracket),
        Token(Ident, "a"),
        Token(Comma),
        Token(Ident, "b"),
        Token(Comma),
        Token(Ident, "c"),
        Token(RightBracket),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "a"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "b"),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "c"),
        Token(EqualEqual),
        Token(Int, "3"),
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
                pattern: List(
                    [
                        Variable(
                            "a",
                        ),
                        Variable(
                            "b",
                        ),
                        Variable(
                            "c",
                        ),
                    ],
                ),
                value: Some(
                    List(
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
                                "a",
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
                                "b",
                            ),
                            Literal(
                                Int(
                                    2,
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
                                "c",
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
                    EvalDeclarePattern {
                        pattern: List(
                            [
                                Variable(
                                    "a",
                                ),
                                Variable(
                                    "b",
                                ),
                                Variable(
                                    "c",
                                ),
                            ],
                        ),
                        init_expr: RustValue(
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
                                                    name: "a",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                                    name: "b",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                                    name: "c",
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
            ],
        },
    ),
)
```