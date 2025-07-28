# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = {"a": 1, "b": 2,};
assert(x["a"] == 1);
assert(x["b"] == 2);
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
        Token(String, "a"),
        Token(Colon),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "b"),
        Token(Colon),
        Token(Int, "2"),
        Token(Comma),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(LeftBracket),
        Token(String, "a"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(LeftBracket),
        Token(String, "b"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "2"),
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
                        [
                            (
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "b",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                        ],
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
                            GetItem(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    String(
                                        "b",
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
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
                            name: "x",
                            init_expr: Some(
                                EvalRef(
                                    EvalDict {
                                        pairs: [
                                            (
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "a",
                                                        ),
                                                    },
                                                ),
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: Int(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            (
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "b",
                                                        ),
                                                    },
                                                ),
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: Int(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ],
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "x",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: String(
                                                                    "a",
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "x",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: String(
                                                                    "b",
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        2,
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