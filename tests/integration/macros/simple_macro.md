# Program
Status: 🔴
Assertions: 0

```rustleaf
#[macro]
fn identity(eval_node) {
    // Just return the node as-is
    eval_node
}

#[identity]
fn test_func() {
    42
}

var result = test_func();
assert(result == 42, "identity macro should preserve function");
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: identity",
)
```

# Lex
```rust
Ok(
    [
        Token(Hash),
        Token(LeftBracket),
        Token(Macro),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "identity"),
        Token(LeftParen),
        Token(Ident, "eval_node"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "eval_node"),
        Token(RightBrace),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "identity"),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "test_func"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Int, "42"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "test_func"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(Comma),
        Token(String, "identity macro should preserve function"),
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
            Macro {
                name: "macro",
                args: [],
                statement: FnDecl {
                    name: "identity",
                    params: [
                        Parameter {
                            name: "eval_node",
                            default: None,
                            kind: Regular,
                        },
                    ],
                    body: Block {
                        statements: [],
                        final_expr: Some(
                            Identifier(
                                "eval_node",
                            ),
                        ),
                    },
                    is_pub: false,
                },
            },
            Macro {
                name: "identity",
                args: [],
                statement: FnDecl {
                    name: "test_func",
                    params: [],
                    body: Block {
                        statements: [],
                        final_expr: Some(
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    },
                    is_pub: false,
                },
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "test_func",
                        ),
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
                        Eq(
                            Identifier(
                                "result",
                            ),
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "identity macro should preserve function",
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
                        EvalMacro {
                            data: MacroData {
                                macro_fn: Eval(
                                    EvalRef(
                                        EvalVariable {
                                            name: "macro",
                                        },
                                    ),
                                ),
                                target: Eval(
                                    EvalRef(
                                        EvalFunction {
                                            data: FunctionData {
                                                name: "identity",
                                                params: [
                                                    (
                                                        "eval_node",
                                                        None,
                                                        Regular,
                                                    ),
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        EvalBlock {
                                                            statements: [],
                                                            final_expr: Some(
                                                                EvalRef(
                                                                    EvalVariable {
                                                                        name: "eval_node",
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ),
                                args: [],
                            },
                        },
                    ),
                    EvalRef(
                        EvalMacro {
                            data: MacroData {
                                macro_fn: Eval(
                                    EvalRef(
                                        EvalVariable {
                                            name: "identity",
                                        },
                                    ),
                                ),
                                target: Eval(
                                    EvalRef(
                                        EvalFunction {
                                            data: FunctionData {
                                                name: "test_func",
                                                params: [],
                                                body: Eval(
                                                    EvalRef(
                                                        EvalBlock {
                                                            statements: [],
                                                            final_expr: Some(
                                                                EvalRef(
                                                                    EvalLiteral {
                                                                        value: Int(
                                                                            42,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ),
                                args: [],
                            },
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "result",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "test_func",
                                            },
                                        ),
                                        args: [],
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
                                                        name: "result",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        42,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                EvalRef(
                                    EvalLiteral {
                                        value: String(
                                            "identity macro should preserve function",
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