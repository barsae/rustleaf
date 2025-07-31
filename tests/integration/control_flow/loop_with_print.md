# Program
Status: 🟢
Assertions: 1

```rustleaf
assert((loop {
    break 42;
}) == 42);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_statement: falling back to expression statement
parse_expression: starting at position 0
parse_expression: starting at position 2
parse_expression: starting at position 3
parse_statement: starting at position 5
parse_expression: starting at position 6
parse_expression: success
parse_statement: parsed break statement
parse_expression: success
parse_expression: success
parse_expression: success
parse_program: parsed 1 statements
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
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalLoop {
                                                    body: RustValue(
                                                        EvalBlock {
                                                            statements: [
                                                                RustValue(
                                                                    EvalBreak {
                                                                        expr: Some(
                                                                            RustValue(
                                                                                EvalLiteral {
                                                                                    value: Int(
                                                                                        42,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                            final_expr: None,
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
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
)
```