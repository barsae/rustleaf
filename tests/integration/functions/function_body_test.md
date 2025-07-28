# Program
Status: 🟢
Assertions: 1

```rustleaf
fn greet() {
    "hello"
}
assert(greet() == "hello");
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
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(String, "hello"),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(EqualEqual),
        Token(String, "hello"),
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
            FnDecl {
                name: "greet",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            String(
                                "hello",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "greet",
                                ),
                                [],
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
                        EvalFunction {
                            data: FunctionData {
                                name: "greet",
                                params: [],
                                body: Eval(
                                    EvalRef(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "hello",
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            },
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
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "greet",
                                                            },
                                                        ),
                                                        args: [],
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: String(
                                                        "hello",
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