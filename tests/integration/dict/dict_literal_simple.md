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
    EvalProgram {
        statements: [
            EvalDeclare {
                name: "x",
                init_expr: Some(
                    EvalDict {
                        pairs: [
                            (
                                EvalLiteral {
                                    value: String(
                                        "a",
                                    ),
                                },
                                EvalLiteral {
                                    value: Int(
                                        1,
                                    ),
                                },
                            ),
                            (
                                EvalLiteral {
                                    value: String(
                                        "b",
                                    ),
                                },
                                EvalLiteral {
                                    value: Int(
                                        2,
                                    ),
                                },
                            ),
                        ],
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
                            obj_expr: EvalGetItem {
                                obj_expr: EvalVariable {
                                    name: "x",
                                },
                                index_expr: EvalLiteral {
                                    value: String(
                                        "a",
                                    ),
                                },
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    1,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalGetItem {
                                obj_expr: EvalVariable {
                                    name: "x",
                                },
                                index_expr: EvalLiteral {
                                    value: String(
                                        "b",
                                    ),
                                },
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    2,
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