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
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "y",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalBlock {
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
                                                                                        10,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_expr: Some(
                                                        EvalRef(
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
                                                                                attr_name: "op_add",
                                                                            },
                                                                        },
                                                                    ),
                                                                    args: [
                                                                        EvalRef(
                                                                            RefCell {
                                                                                value: EvalLiteral {
                                                                                    value: Int(
                                                                                        5,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
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
                                                                            name: "y",
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
                                                                        15,
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