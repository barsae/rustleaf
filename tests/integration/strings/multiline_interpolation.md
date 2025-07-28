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
                            EvalLiteral {
                                value: String(
                                    "!\nThis is a multiline string with interpolation.",
                                ),
                            },
                        ],
                    },
                ),
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "print",
                },
                args: [
                    EvalVariable {
                        name: "result",
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
                                name: "result",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "Hello World!\nThis is a multiline string with interpolation.",
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