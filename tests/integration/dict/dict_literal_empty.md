# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = {};
assert(x is Dict);
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
        Token(Ident, "x"),
        Token(Equal),
        Token(LeftBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Is),
        Token(Ident, "Dict"),
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
                    "x",
                ),
                value: Some(
                    Dict(
                        [],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "Dict",
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
                            name: "x",
                            init_expr: Some(
                                EvalRef(
                                    EvalDict {
                                        pairs: [],
                                    },
                                ),
                            ),
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
                                    EvalIs {
                                        left: EvalRef(
                                            EvalVariable {
                                                name: "x",
                                            },
                                        ),
                                        right: EvalRef(
                                            EvalVariable {
                                                name: "Dict",
                                            },
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