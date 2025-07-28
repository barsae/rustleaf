# Program
Status: 🟢
Assertions: 1

```rustleaf
var y = {
    var x = 10;
    x + 5
};
assert(y == 15);
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
        Token(Ident, "y"),
        Token(Equal),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "5"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(EqualEqual),
        Token(Int, "15"),
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
                    "y",
                ),
                value: Some(
                    Block(
                        Block {
                            statements: [
                                VarDecl {
                                    pattern: Variable(
                                        "x",
                                    ),
                                    value: Some(
                                        Literal(
                                            Int(
                                                10,
                                            ),
                                        ),
                                    ),
                                },
                            ],
                            final_expr: Some(
                                Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "y",
                            ),
                            Literal(
                                Int(
                                    15,
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
                            name: "y",
                            init_expr: Some(
                                EvalRef(
                                    EvalBlock {
                                        statements: [
                                            EvalRef(
                                                EvalDeclare {
                                                    name: "x",
                                                    init_expr: Some(
                                                        EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    10,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ],
                                        final_expr: Some(
                                            EvalRef(
                                                EvalCall {
                                                    func_expr: EvalRef(
                                                        EvalGetAttr {
                                                            obj_expr: EvalRef(
                                                                EvalVariable {
                                                                    name: "x",
                                                                },
                                                            ),
                                                            attr_name: "op_add",
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ),
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
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalVariable {
                                                        name: "y",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        15,
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