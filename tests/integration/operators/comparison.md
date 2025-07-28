# Program
Status: 🟢
Assertions: 10

```rustleaf
// Basic comparison tests
assert(3 == 3);
assert(3 != 4);
assert(3 < 4);
assert(4 > 3);
assert(3 <= 3);
assert(3 >= 3);

// Mixed numeric comparisons
assert(3 == 3.0);
assert(3.5 > 3);

// String comparisons
assert("a" < "b");
assert("hello" == "hello");
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
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(BangEqual),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(Less),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Greater),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(LessEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(GreaterEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Float, "3.0"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Float, "3.5"),
        Token(Greater),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "a"),
        Token(Less),
        Token(String, "b"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(EqualEqual),
        Token(String, "hello"),
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
                            Literal(
                                Int(
                                    3,
                                ),
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
                        Ne(
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
                        Lt(
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
                        Gt(
                            Literal(
                                Int(
                                    4,
                                ),
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
                        Le(
                            Literal(
                                Int(
                                    3,
                                ),
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
                        Ge(
                            Literal(
                                Int(
                                    3,
                                ),
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
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Float(
                                    3.0,
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
                        Gt(
                            Literal(
                                Float(
                                    3.5,
                                ),
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
                        Lt(
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Literal(
                                String(
                                    "b",
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
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello",
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
                            obj_expr: EvalLiteral {
                                value: Int(
                                    3,
                                ),
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
                            obj_expr: EvalLiteral {
                                value: Int(
                                    3,
                                ),
                            },
                            attr_name: "op_ne",
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
                            obj_expr: EvalLiteral {
                                value: Int(
                                    3,
                                ),
                            },
                            attr_name: "op_lt",
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
                            obj_expr: EvalLiteral {
                                value: Int(
                                    4,
                                ),
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
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: Int(
                                    3,
                                ),
                            },
                            attr_name: "op_le",
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
                            obj_expr: EvalLiteral {
                                value: Int(
                                    3,
                                ),
                            },
                            attr_name: "op_ge",
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
                            obj_expr: EvalLiteral {
                                value: Int(
                                    3,
                                ),
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Float(
                                    3.0,
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
                            obj_expr: EvalLiteral {
                                value: Float(
                                    3.5,
                                ),
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
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: String(
                                    "a",
                                ),
                            },
                            attr_name: "op_lt",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "b",
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
                            obj_expr: EvalLiteral {
                                value: String(
                                    "hello",
                                ),
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "hello",
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