# Program
Status: 🟢
Assertions: 1

```rustleaf
var i;
try {
    raise(42);
} catch e {
    i = e;
}
assert(i == 42);
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
        Token(Ident, "i"),
        Token(Semicolon),
        Token(Try),
        Token(LeftBrace),
        Token(Ident, "raise"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(Catch),
        Token(Ident, "e"),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Equal),
        Token(Ident, "e"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "i"),
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
            VarDecl {
                pattern: Variable(
                    "i",
                ),
                value: None,
            },
            Expression(
                Try {
                    body: Block {
                        statements: [
                            Expression(
                                FunctionCall(
                                    Identifier(
                                        "raise",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                42,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        ],
                        final_expr: None,
                    },
                    catch: CatchClause {
                        pattern: Variable(
                            "e",
                        ),
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "i",
                                    ),
                                    op: Assign,
                                    value: Identifier(
                                        "e",
                                    ),
                                },
                            ],
                            final_expr: None,
                        },
                    },
                },
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "i",
                            ),
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
    Eval(
        EvalRef(
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "i",
                                    init_expr: None,
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalTry {
                                    body: EvalRef(
                                        RefCell {
                                            value: EvalBlock {
                                                statements: [
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalCall {
                                                                func_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "raise",
                                                                        },
                                                                    },
                                                                ),
                                                                args: [
                                                                    EvalRef(
                                                                        RefCell {
                                                                            value: EvalLiteral {
                                                                                value: Int(
                                                                                    42,
                                                                                ),
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_expr: None,
                                            },
                                        },
                                    ),
                                    catch_pattern: Variable(
                                        "e",
                                    ),
                                    catch_body: EvalRef(
                                        RefCell {
                                            value: EvalBlock {
                                                statements: [
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalAssign {
                                                                name: "i",
                                                                expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "e",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_expr: None,
                                            },
                                        },
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "i",
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        42,
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
                    ],
                },
            },
        ),
    ),
)
```