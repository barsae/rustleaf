# Program
Status: 🟢
Assertions: 4

```rustleaf
assert(1 + 2 == 3);
assert(5 - 3 == 2);
assert(4 * 3 == 12);
assert(10 / 2 == 5.0);
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
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Minus),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Star),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "12"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Slash),
        Token(Int, "2"),
        Token(EqualEqual),
        Token(Float, "5.0"),
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
                            Add(
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
                            Sub(
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
                            Mul(
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
                            Literal(
                                Int(
                                    12,
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
                            Div(
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    5.0,
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
    RustValueRef(
        RefCell {
            value: EvalProgram {
                statements: [
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalCall {
                                                                        func_expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalGetAttr {
                                                                                    obj_expr: RustValueRef(
                                                                                        RefCell {
                                                                                            value: EvalLiteral {
                                                                                                value: Int(
                                                                                                    1,
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "op_add",
                                                                                },
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalLiteral {
                                                                                        value: Int(
                                                                                            2,
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    3,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                            },
                                        },
                                    ),
                                ],
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalCall {
                                                                        func_expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalGetAttr {
                                                                                    obj_expr: RustValueRef(
                                                                                        RefCell {
                                                                                            value: EvalLiteral {
                                                                                                value: Int(
                                                                                                    5,
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "op_sub",
                                                                                },
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalLiteral {
                                                                                        value: Int(
                                                                                            3,
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    2,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                            },
                                        },
                                    ),
                                ],
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalCall {
                                                                        func_expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalGetAttr {
                                                                                    obj_expr: RustValueRef(
                                                                                        RefCell {
                                                                                            value: EvalLiteral {
                                                                                                value: Int(
                                                                                                    4,
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "op_mul",
                                                                                },
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalLiteral {
                                                                                        value: Int(
                                                                                            3,
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    12,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                            },
                                        },
                                    ),
                                ],
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalCall {
                                                                        func_expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalGetAttr {
                                                                                    obj_expr: RustValueRef(
                                                                                        RefCell {
                                                                                            value: EvalLiteral {
                                                                                                value: Int(
                                                                                                    10,
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "op_div",
                                                                                },
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalLiteral {
                                                                                        value: Int(
                                                                                            2,
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Float(
                                                                    5.0,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                            },
                                        },
                                    ),
                                ],
                            },
                        },
                    ),
                ],
            },
        },
    ),
)
```