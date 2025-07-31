# Program
Status: 🟢
Assertions: 2

```rustleaf
var [first, *rest] = [1, 2, 3, 4];
assert(first == 1);
assert(rest == [2, 3, 4]);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_expression: starting at position 8
parse_expression: starting at position 9
parse_expression: success
parse_expression: starting at position 11
parse_expression: success
parse_expression: starting at position 13
parse_expression: success
parse_expression: starting at position 15
parse_expression: success
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 18
parse_statement: starting at position 18
parse_statement: falling back to expression statement
parse_expression: starting at position 18
parse_expression: starting at position 20
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 25
parse_statement: starting at position 25
parse_statement: falling back to expression statement
parse_expression: starting at position 25
parse_expression: starting at position 27
parse_expression: starting at position 30
parse_expression: success
parse_expression: starting at position 32
parse_expression: success
parse_expression: starting at position 34
parse_expression: success
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
        Token(LeftBracket),
        Token(Ident, "first"),
        Token(Comma),
        Token(Star),
        Token(Ident, "rest"),
        Token(RightBracket),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(Int, "4"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "first"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "rest"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(Int, "4"),
        Token(RightBracket),
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
                pattern: ListRest(
                    [
                        Variable(
                            "first",
                        ),
                    ],
                    Some(
                        "rest",
                    ),
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
                            Literal(
                                Int(
                                    4,
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
                                "first",
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
                                "rest",
                            ),
                            List(
                                [
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
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
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
                        pattern: ListRest(
                            [
                                Variable(
                                    "first",
                                ),
                            ],
                            Some(
                                "rest",
                            ),
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
                                    RustValue(
                                        EvalLiteral {
                                            value: Int(
                                                4,
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
                                                    name: "first",
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
                                                    name: "rest",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [
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
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                4,
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
)
```