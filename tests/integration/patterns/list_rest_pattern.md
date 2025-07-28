# Program
Status: 🟢
Assertions: 2

```rustleaf
var [first, *rest] = [1, 2, 3, 4];
assert(first == 1);
assert(rest == [2, 3, 4]);
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
    RustValueRef(
        RefCell {
            value: EvalProgram {
                statements: [
                    RustValueRef(
                        RefCell {
                            value: EvalDeclarePattern {
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
                                init_expr: RustValueRef(
                                    RefCell {
                                        value: EvalList {
                                            elements: [
                                                RustValueRef(
                                                    RefCell {
                                                        value: EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    },
                                                ),
                                                RustValueRef(
                                                    RefCell {
                                                        value: EvalLiteral {
                                                            value: Int(
                                                                2,
                                                            ),
                                                        },
                                                    },
                                                ),
                                                RustValueRef(
                                                    RefCell {
                                                        value: EvalLiteral {
                                                            value: Int(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                ),
                                                RustValueRef(
                                                    RefCell {
                                                        value: EvalLiteral {
                                                            value: Int(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ],
                                        },
                                    },
                                ),
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
                                                                    value: EvalVariable {
                                                                        name: "first",
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
                                                                    1,
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
                                                                    value: EvalVariable {
                                                                        name: "rest",
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
                                                            value: EvalList {
                                                                elements: [
                                                                    RustValueRef(
                                                                        RefCell {
                                                                            value: EvalLiteral {
                                                                                value: Int(
                                                                                    2,
                                                                                ),
                                                                            },
                                                                        },
                                                                    ),
                                                                    RustValueRef(
                                                                        RefCell {
                                                                            value: EvalLiteral {
                                                                                value: Int(
                                                                                    3,
                                                                                ),
                                                                            },
                                                                        },
                                                                    ),
                                                                    RustValueRef(
                                                                        RefCell {
                                                                            value: EvalLiteral {
                                                                                value: Int(
                                                                                    4,
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
                ],
            },
        },
    ),
)
```