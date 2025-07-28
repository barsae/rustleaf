# Program
Status: 🟢
Assertions: 7

```rustleaf
// Bitwise operators test
assert((5 & 3) == 1);  // 101 & 011 = 001
assert((5 | 3) == 7);  // 101 | 011 = 111
assert((5 ^ 3) == 6);  // 101 ^ 011 = 110

// Bit shifts
assert((8 << 1) == 16);
assert((8 >> 1) == 4);
assert((8 >> 2) == 2);

// Bitwise NOT
assert((~5) == -6);  // Two's complement
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
        Token(LeftParen),
        Token(Int, "5"),
        Token(Ampersand),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Pipe),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "7"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Caret),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "8"),
        Token(LessLess),
        Token(Int, "1"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "16"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "8"),
        Token(GreaterGreater),
        Token(Int, "1"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Int, "8"),
        Token(GreaterGreater),
        Token(Int, "2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(Tilde),
        Token(Int, "5"),
        Token(RightParen),
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
                            BitAnd(
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
                            BitOr(
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
                                    7,
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
                            BitXor(
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
                                    6,
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
                            LeftShift(
                                Literal(
                                    Int(
                                        8,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    16,
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
                            RightShift(
                                Literal(
                                    Int(
                                        8,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
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
                        Eq(
                            RightShift(
                                Literal(
                                    Int(
                                        8,
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
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                    attr_name: "op_bitwise_and",
                                },
                                args: [
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
                                    1,
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
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                    attr_name: "op_bitwise_or",
                                },
                                args: [
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
                                    7,
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
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                    attr_name: "op_bitwise_xor",
                                },
                                args: [
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
                                    6,
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
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            8,
                                        ),
                                    },
                                    attr_name: "op_lshift",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            1,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    16,
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
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            8,
                                        ),
                                    },
                                    attr_name: "op_rshift",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            1,
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
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            8,
                                        ),
                                    },
                                    attr_name: "op_rshift",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            2,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalCall {
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                    attr_name: "op_bitwise_not",
                                },
                                args: [],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalCall {
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            6,
                                        ),
                                    },
                                    attr_name: "op_neg",
                                },
                                args: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
)
```