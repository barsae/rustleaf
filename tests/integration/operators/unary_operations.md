# Program
Status: 🟢
Assertions: 4

```rustleaf
assert(-42 == -42);
assert(not true == false);
assert(not false == true);
assert(~5 == -6);
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
        Token(Minus),
        Token(Int, "42"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(True),
        Token(EqualEqual),
        Token(False),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(False),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Tilde),
        Token(Int, "5"),
        Token(EqualEqual),
        Token(Minus),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Neg(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        42,
                                    ),
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
                            Not(
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    false,
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
                            Not(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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
                            BitNot(
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        6,
                                    ),
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
    Eval(
        EvalRef(
            EvalProgram {
                statements: [
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalLiteral {
                                                                        value: Int(
                                                                            42,
                                                                        ),
                                                                    },
                                                                ),
                                                                attr_name: "op_neg",
                                                            },
                                                        ),
                                                        args: [],
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalCall {
                                                    func_expr: EvalRef(
                                                        EvalGetAttr {
                                                            obj_expr: EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        42,
                                                                    ),
                                                                },
                                                            ),
                                                            attr_name: "op_neg",
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalLogicalNot {
                                                        expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Bool(
                                                                    true,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Bool(
                                                        false,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalLogicalNot {
                                                        expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Bool(
                                                                    false,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Bool(
                                                        true,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalLiteral {
                                                                        value: Int(
                                                                            5,
                                                                        ),
                                                                    },
                                                                ),
                                                                attr_name: "op_bitwise_not",
                                                            },
                                                        ),
                                                        args: [],
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalCall {
                                                    func_expr: EvalRef(
                                                        EvalGetAttr {
                                                            obj_expr: EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        6,
                                                                    ),
                                                                },
                                                            ),
                                                            attr_name: "op_neg",
                                                        },
                                                    ),
                                                    args: [],
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
)
```