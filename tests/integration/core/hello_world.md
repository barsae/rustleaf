# Program
Status: 🟢
Assertions: 1

```rustleaf
assert("hello, world" == "hello, world");
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello, world"),
        Token(EqualEqual),
        Token(String, "hello, world"),
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
                        Eq(
                            Literal(
                                String(
                                    "hello, world",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello, world",
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: String(
                                    "hello, world",
                                ),
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "hello, world",
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