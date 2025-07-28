# Program
Status: 🟢
Assertions: 4

```rustleaf
var x = 42;
var y = "hello";
var z = true;
assert(x == 42);
assert(y == "hello");
assert(z == true);
assert(x + 8 == 50);
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "y"),
        Token(Equal),
        Token(String, "hello"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "z"),
        Token(Equal),
        Token(True),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(EqualEqual),
        Token(String, "hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "8"),
        Token(EqualEqual),
        Token(Int, "50"),
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
                    "x",
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
                    "y",
                ),
                value: Some(
                    Literal(
                        String(
                            "hello",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: Some(
                    Literal(
                        Bool(
                            true,
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
                                "x",
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
                            Identifier(
                                "y",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "z",
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
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        8,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    50,
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
                name: "x",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            42,
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "y",
                init_expr: Some(
                    EvalLiteral {
                        value: String(
                            "hello",
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "z",
                init_expr: Some(
                    EvalLiteral {
                        value: Bool(
                            true,
                        ),
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
                                name: "x",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    42,
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
                                name: "y",
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "z",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Bool(
                                    true,
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
                                    obj_expr: EvalVariable {
                                        name: "x",
                                    },
                                    attr_name: "op_add",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            8,
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    50,
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