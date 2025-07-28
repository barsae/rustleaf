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
    EvalProgram {
        statements: [
            EvalDeclare {
                name: "y",
                init_expr: Some(
                    EvalBlock {
                        statements: [
                            EvalDeclare {
                                name: "x",
                                init_expr: Some(
                                    EvalLiteral {
                                        value: Int(
                                            10,
                                        ),
                                    },
                                ),
                            },
                        ],
                        final_expr: Some(
                            EvalCall {
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalVariable {
                                        name: "x",
                                    },
                                    attr_name: "op_add",
                                },
                                args: [
                                    EvalLiteral {
                                        value: Int(
                                            5,
                                        ),
                                    },
                                ],
                            },
                        ),
                    },
                ),
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "y",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    15,
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