# Program
Status: 🟢
Assertions: 1

```rustleaf
assert((loop {
    break 42;
}) == 42);
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
        Token(LeftParen),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "42"),
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
                            Loop {
                                body: Block {
                                    statements: [
                                        Break(
                                            Some(
                                                Literal(
                                                    Int(
                                                        42,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                    final_expr: None,
                                },
                            },
                            Literal(
                                Int(
                                    42,
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
                            obj_expr: EvalLoop {
                                body: EvalBlock {
                                    statements: [
                                        EvalBreak {
                                            expr: Some(
                                                EvalLiteral {
                                                    value: Int(
                                                        42,
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                    final_expr: None,
                                },
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
        ],
    },
)
```