# Program
Status: 🟢
Assertions: 1

```rustleaf
var s = 0;
for x in [1, 2, 3] {
    s += x;
}
assert(s == 6);
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
        Token(Ident, "s"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "x"),
        Token(In),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(LeftBrace),
        Token(Ident, "s"),
        Token(PlusEqual),
        Token(Ident, "x"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "s"),
        Token(EqualEqual),
        Token(Int, "6"),
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
                    "s",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "s",
                                ),
                                op: AddAssign,
                                value: Identifier(
                                    "x",
                                ),
                            },
                        ],
                        final_expr: None,
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
                                "s",
                            ),
                            Literal(
                                Int(
                                    6,
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
                name: "s",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            0,
                        ),
                    },
                ),
            },
            EvalFor {
                var_name: "x",
                iter_expr: EvalList {
                    elements: [
                        EvalLiteral {
                            value: Int(
                                1,
                            ),
                        },
                        EvalLiteral {
                            value: Int(
                                2,
                            ),
                        },
                        EvalLiteral {
                            value: Int(
                                3,
                            ),
                        },
                    ],
                },
                body: EvalBlock {
                    statements: [
                        EvalAssign {
                            name: "s",
                            expr: EvalCall {
                                func_expr: EvalGetAttr {
                                    obj_expr: EvalVariable {
                                        name: "s",
                                    },
                                    attr_name: "op_add",
                                },
                                args: [
                                    EvalVariable {
                                        name: "x",
                                    },
                                ],
                            },
                        },
                    ],
                    final_expr: None,
                },
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "s",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    6,
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