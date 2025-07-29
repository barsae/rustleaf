# Program
Status: 🟢
Assertions: 7

```rustleaf
assert(sqrt(4) == 2.0);
assert(abs(-5) == 5);
assert(floor(3.7) == 3);
assert(ceil(3.2) == 4);
assert(round(3.5) == 4);
assert(min(5, 3) == 3);
assert(max(5, 3) == 5);
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sqrt"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Float, "2.0"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "abs"),
        Token(LeftParen),
        Token(Minus),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "floor"),
        Token(LeftParen),
        Token(Float, "3.7"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "ceil"),
        Token(LeftParen),
        Token(Float, "3.2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "round"),
        Token(LeftParen),
        Token(Float, "3.5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "min"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "max"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "sqrt",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Float(
                                    2.0,
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
                                    "abs",
                                ),
                                [
                                    Neg(
                                        Literal(
                                            Int(
                                                5,
                                            ),
                                        ),
                                    ),
                                ],
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
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "floor",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.7,
                                        ),
                                    ),
                                ],
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "ceil",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.2,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    4,
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
                                    "round",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.5,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    4,
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
                                    "min",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "max",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
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
        ],
    ),
)
```

# Eval
```rust
Ok(
    EvalProgram {
        statements: [
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "sqrt",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            4,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Float(
                                    2.0,
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
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "abs",
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
                        func_expr: EvalGetAttr {
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "floor",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Float(
                                            3.7,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    3,
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
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "ceil",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Float(
                                            3.2,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    4,
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
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "round",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Float(
                                            3.5,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    4,
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
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "min",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                    EvalLiteral {
                                        value: Int(
                                            3,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    3,
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
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "max",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                    EvalLiteral {
                                        value: Int(
                                            3,
                                        ),
                                    },
                                ],
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
        ],
    },
)
```