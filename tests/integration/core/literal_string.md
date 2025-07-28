# Program
Status: 🟢
Assertions: 3

```rustleaf
var greeting = "hello";
var empty = "";
assert(greeting == "hello");
assert(empty == "");
assert(greeting + " world" == "hello world");
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
        Token(Ident, "greeting"),
        Token(Equal),
        Token(String, "hello"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "empty"),
        Token(Equal),
        Token(String, ""),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting"),
        Token(EqualEqual),
        Token(String, "hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "empty"),
        Token(EqualEqual),
        Token(String, ""),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting"),
        Token(Plus),
        Token(String, " world"),
        Token(EqualEqual),
        Token(String, "hello world"),
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
                    "greeting",
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
                    "empty",
                ),
                value: Some(
                    Literal(
                        String(
                            "",
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
                                "greeting",
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
                                "empty",
                            ),
                            Literal(
                                String(
                                    "",
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
                                    "greeting",
                                ),
                                Literal(
                                    String(
                                        " world",
                                    ),
                                ),
                            ),
                            Literal(
                                String(
                                    "hello world",
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
                name: "greeting",
                init_expr: Some(
                    EvalLiteral {
                        value: String(
                            "hello",
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "empty",
                init_expr: Some(
                    EvalLiteral {
                        value: String(
                            "",
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
                                name: "greeting",
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
                                name: "empty",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "",
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
                                        name: "greeting",
                                    },
                                    attr_name: "op_add",
                                },
                                args: [
                                    EvalLiteral {
                                        value: String(
                                            " world",
                                        ),
                                    },
                                ],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "hello world",
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