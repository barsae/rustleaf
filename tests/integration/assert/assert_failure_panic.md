# Program
Status: 🟢
Assertions: 1

```rustleaf
assert(false, "This should fail");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_statement: falling back to expression statement
parse_expression: starting at position 0
parse_expression: starting at position 2
parse_expression: success
parse_expression: starting at position 4
parse_expression: success
parse_expression: success
parse_program: parsed 1 statements
```

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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalLiteral {
                                    value: Bool(
                                        false,
                                    ),
                                },
                            ),
                            RustValue(
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
)
```