# Program
Status: 🟢
Assertions: 1

```rustleaf
fn test(x, y) {
    x + y
}

var z = 1 : test(2);
assert(z == 3);
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
        Token(Ident, "test"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "z"),
        Token(Equal),
        Token(Int, "1"),
        Token(Colon),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
        Token(EqualEqual),
        Token(Int, "3"),
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
                name: "test",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: Some(
                    Pipe(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        FunctionCall(
                            Identifier(
                                "test",
                            ),
                            [
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ],
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
                            Identifier(
                                "z",
                            ),
                            Literal(
                                Int(
                                    3,
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
    RustValueRef(
        RefCell {
            value: EvalProgram {
                statements: [
                    RustValueRef(
                        RefCell {
                            value: EvalFunction {
                                data: FunctionData {
                                    name: "test",
                                    params: [
                                        (
                                            "x",
                                            None,
                                            Regular,
                                        ),
                                        (
                                            "y",
                                            None,
                                            Regular,
                                        ),
                                    ],
                                    body: RustValueRef(
                                        RefCell {
                                            value: EvalBlock {
                                                statements: [],
                                                final_expr: Some(
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalCall {
                                                                func_expr: RustValueRef(
                                                                    RefCell {
                                                                        value: EvalGetAttr {
                                                                            obj_expr: RustValueRef(
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
                                                                    RustValueRef(
                                                                        RefCell {
                                                                            value: EvalVariable {
                                                                                name: "y",
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
                                },
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "z",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalVariable {
                                                            name: "test",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    2,
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
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "z",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    3,
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
)
```