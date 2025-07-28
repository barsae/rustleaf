# Program
Status: 🟢
Assertions: 1

```rustleaf
assert(false, "This should fail");
```

# Output
None

# Result
```rust
Err(
    "Assertion failed: This should fail",
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(False),
        Token(Comma),
        Token(String, "This should fail"),
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
                        Literal(
                            Bool(
                                false,
                            ),
                        ),
                        Literal(
                            String(
                                "This should fail",
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
                                    EvalLiteral {
                                        value: Bool(
                                            false,
                                        ),
                                    },
                                ),
                                EvalRef(
                                    EvalLiteral {
                                        value: String(
                                            "This should fail",
                                        ),
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