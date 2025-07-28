# Program
Status: 🟢
Assertions: 1

```rustleaf
fn test_return() {
    return 42;
}

var result = test_return();
assert(result == 42);
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
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Return),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
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
            FnDecl {
                name: "test_return",
                params: [],
                body: Block {
                    statements: [
                        Return(
                            Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "test_return",
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
            EvalFunction {
                data: FunctionData {
                    name: "test_return",
                    params: [],
                    body: EvalBlock {
                        statements: [
                            EvalReturn {
                                expr: Some(
                                    EvalLiteral {
                                        value: Int(
                                            42,
                                        ),
                                    },
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                },
            },
            EvalDeclare {
                name: "result",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalVariable {
                            name: "test_return",
                        },
                        args: [],
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
                                name: "result",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    42,
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