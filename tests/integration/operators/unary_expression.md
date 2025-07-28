# Program
Status: 🟢
Assertions: 4

```rustleaf
var positive = 42;
var negative = -positive;
var double_neg = -negative;
assert(negative == -42);
assert(double_neg == 42);
assert(-100 == -100);
assert(-(5 + 3) == -8);
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
        Token(Ident, "positive"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "negative"),
        Token(Equal),
        Token(Minus),
        Token(Ident, "positive"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "double_neg"),
        Token(Equal),
        Token(Minus),
        Token(Ident, "negative"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "negative"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "double_neg"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Minus),
        Token(Int, "100"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "100"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Minus),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Plus),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "8"),
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
                    "positive",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "negative",
                ),
                value: Some(
                    Neg(
                        Identifier(
                            "positive",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "double_neg",
                ),
                value: Some(
                    Neg(
                        Identifier(
                            "negative",
                        ),
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
                                "negative",
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
                            Identifier(
                                "double_neg",
                            ),
                            Literal(
                                Int(
                                    42,
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
                            Neg(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        100,
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
                            Neg(
                                Add(
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
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        8,
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
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "positive",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: Int(
                                                        42,
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "negative",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "positive",
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_neg",
                                                            },
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "double_neg",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "negative",
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_neg",
                                                            },
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "negative",
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalCall {
                                                                    func_expr: EvalRef(
                                                                        RefCell {
                                                                            value: EvalGetAttr {
                                                                                obj_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                42,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                attr_name: "op_neg",
                                                                            },
                                                                        },
                                                                    ),
                                                                    args: [],
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
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "double_neg",
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        42,
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
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalCall {
                                                                            func_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalGetAttr {
                                                                                        obj_expr: EvalRef(
                                                                                            RefCell {
                                                                                                value: EvalLiteral {
                                                                                                    value: Int(
                                                                                                        100,
                                                                                                    ),
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        attr_name: "op_neg",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            args: [],
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalCall {
                                                                    func_expr: EvalRef(
                                                                        RefCell {
                                                                            value: EvalGetAttr {
                                                                                obj_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                100,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                attr_name: "op_neg",
                                                                            },
                                                                        },
                                                                    ),
                                                                    args: [],
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
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalCall {
                                                                            func_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalGetAttr {
                                                                                        obj_expr: EvalRef(
                                                                                            RefCell {
                                                                                                value: EvalCall {
                                                                                                    func_expr: EvalRef(
                                                                                                        RefCell {
                                                                                                            value: EvalGetAttr {
                                                                                                                obj_expr: EvalRef(
                                                                                                                    RefCell {
                                                                                                                        value: EvalLiteral {
                                                                                                                            value: Int(
                                                                                                                                5,
                                                                                                                            ),
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                attr_name: "op_add",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    args: [
                                                                                                        EvalRef(
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
                                                                                        attr_name: "op_neg",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            args: [],
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalCall {
                                                                    func_expr: EvalRef(
                                                                        RefCell {
                                                                            value: EvalGetAttr {
                                                                                obj_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalLiteral {
                                                                                            value: Int(
                                                                                                8,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                attr_name: "op_neg",
                                                                            },
                                                                        },
                                                                    ),
                                                                    args: [],
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
    ),
)
```