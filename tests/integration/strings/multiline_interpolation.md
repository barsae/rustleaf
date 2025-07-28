# Program
Status: 🟢
Assertions: 1

```rustleaf
var name = "World";
var result = "Hello ${name}!
This is a multiline string with interpolation.";
print(result);
assert(result == "Hello World!
This is a multiline string with interpolation.");
```

# Output
```
Hello World!
This is a multiline string with interpolation.
```

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
        Token(StringPart, "!\nThis is a multiline string with interpolation."),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "Hello World!\nThis is a multiline string with interpolation."),
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
                            Text(
                                "!\nThis is a multiline string with interpolation.",
                            ),
                        ],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Identifier(
                            "result",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "Hello World!\nThis is a multiline string with interpolation.",
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
                        EvalDeclare {
                            name: "name",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: String(
                                            "World",
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "result",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "Hello ",
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_add",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalCall {
                                                    func_expr: EvalRef(
                                                        EvalVariable {
                                                            name: "str",
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            EvalVariable {
                                                                name: "name",
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            EvalRef(
                                                EvalLiteral {
                                                    value: String(
                                                        "!\nThis is a multiline string with interpolation.",
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "print",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalVariable {
                                        name: "result",
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
                                                    EvalVariable {
                                                        name: "result",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: String(
                                                        "Hello World!\nThis is a multiline string with interpolation.",
                                                    ),
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