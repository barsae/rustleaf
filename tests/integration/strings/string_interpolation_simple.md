# Program
Status: 🟢
Assertions: 1

```rustleaf
var name = "World";
var result = "Hello ${name}";
assert(result == "Hello World");
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
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "World"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(StringPart, "Hello "),
        Token(InterpolationStart),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "Hello World"),
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
                    "name",
                ),
                value: Some(
                    Literal(
                        String(
                            "World",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Text(
                                "Hello ",
                            ),
                            Expression(
                                Identifier(
                                    "name",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "Hello World",
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
                name: "name",
                init_expr: Some(
                    EvalLiteral {
                        value: String(
                            "World",
                        ),
                    },
                ),
            },
            EvalDeclare {
                name: "result",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: String(
                                    "Hello ",
                                ),
                            },
                            attr_name: "op_add",
                        },
                        args: [
                            EvalCall {
                                func_expr: EvalVariable {
                                    name: "str",
                                },
                                args: [
                                    EvalVariable {
                                        name: "name",
                                    },
                                ],
                            },
                        ],
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
                                name: "result",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "Hello World",
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