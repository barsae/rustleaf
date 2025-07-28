# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 5;
assert((if x > 0 { "positive" } else { "zero or negative" }) == "positive");
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
        Token(Int, "5"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(LeftParen),
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "positive"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "zero or negative"),
        Token(RightBrace),
        Token(RightParen),
        Token(EqualEqual),
        Token(String, "positive"),
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
                    Literal(
                        Int(
                            5,
                        ),
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
                            If {
                                condition: Gt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ),
                                then_expr: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "positive",
                                            ),
                                        ),
                                    ),
                                },
                                else_expr: Some(
                                    Block {
                                        statements: [],
                                        final_expr: Some(
                                            Literal(
                                                String(
                                                    "zero or negative",
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            },
                            Literal(
                                String(
                                    "positive",
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
                                    name: "x",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: Int(
                                                        5,
                                                    ),
                                                },
                                            },
                                        ),
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
                                                                        value: EvalIf {
                                                                            condition: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalCall {
                                                                                        func_expr: EvalRef(
                                                                                            RefCell {
                                                                                                value: EvalGetAttr {
                                                                                                    obj_expr: EvalRef(
                                                                                                        RefCell {
                                                                                                            value: EvalVariable {
                                                                                                                name: "x",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    attr_name: "op_gt",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        args: [
                                                                                            EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalLiteral {
                                                                                                        value: Int(
                                                                                                            0,
                                                                                                        ),
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                    },
                                                                                },
                                                                            ),
                                                                            then_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalBlock {
                                                                                        statements: [],
                                                                                        final_expr: Some(
                                                                                            EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalLiteral {
                                                                                                        value: String(
                                                                                                            "positive",
                                                                                                        ),
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            ),
                                                                            else_expr: Some(
                                                                                EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalBlock {
                                                                                            statements: [],
                                                                                            final_expr: Some(
                                                                                                EvalRef(
                                                                                                    RefCell {
                                                                                                        value: EvalLiteral {
                                                                                                            value: String(
                                                                                                                "zero or negative",
                                                                                                            ),
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
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
                                                                    value: String(
                                                                        "positive",
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