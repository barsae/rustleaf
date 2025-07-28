# Program
Status: 🟢
Assertions: 4

```rustleaf
var num = 123;
var zero = 0;
var negative = -42;
assert(num == 123);
assert(zero == 0);
assert(negative == -42);
assert(num + zero == 123);
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
        Token(Ident, "num"),
        Token(Equal),
        Token(Int, "123"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "zero"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "negative"),
        Token(Equal),
        Token(Minus),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "num"),
        Token(EqualEqual),
        Token(Int, "123"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "zero"),
        Token(EqualEqual),
        Token(Int, "0"),
        Token(RightParen),
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
        Token(Ident, "num"),
        Token(Plus),
        Token(Ident, "zero"),
        Token(EqualEqual),
        Token(Int, "123"),
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
                    "num",
                ),
                value: Some(
                    Literal(
                        Int(
                            123,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "zero",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
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
                        Literal(
                            Int(
                                42,
                            ),
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
                                "num",
                            ),
                            Literal(
                                Int(
                                    123,
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
                                "zero",
                            ),
                            Literal(
                                Int(
                                    0,
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
                            Add(
                                Identifier(
                                    "num",
                                ),
                                Identifier(
                                    "zero",
                                ),
                            ),
                            Literal(
                                Int(
                                    123,
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
            EvalDeclare {
                name: "num",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            123,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "zero",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            0,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "negative",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: Int(
                                    42,
                                ),
                            },
                            attr_name: "op_neg",
                        },
                        args: [],
                    },
                ),
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "num",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    123,
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
                            obj_expr: EvalVariable {
                                name: "zero",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    0,
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
                            obj_expr: EvalVariable {
                                name: "negative",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalCall {
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalLiteral {
                                        value: Int(
                                            42,
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalCall {
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalVariable {
                                        name: "num",
                                    },
                                    attr_name: "op_add",
                                },
                                args: [
                                    EvalVariable {
                                        name: "zero",
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    123,
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