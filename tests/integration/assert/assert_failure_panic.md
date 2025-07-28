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
    RustValueRef(
        RefCell {
            value: EvalProgram {
                statements: [
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
                                            value: EvalLiteral {
                                                value: Bool(
                                                    false,
                                                ),
                                            },
                                        },
                                    ),
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: String(
                                                    "This should fail",
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
)
```