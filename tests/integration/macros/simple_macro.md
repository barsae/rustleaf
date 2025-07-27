# Program
Status: 🟢
Assertions: 1

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
Ok(
    Unit,
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
    Program(
        [
            Macro {
                macro_fn: Variable(
                    "macro",
                ),
                target: Function(
                    "identity",
                    [
                        (
                            "eval_node",
                            None,
                            Regular,
                        ),
                    ],
                    Block(
                        [],
                        Some(
                            Variable(
                                "eval_node",
                            ),
                        ),
                    ),
                ),
                args: [],
            },
            Macro {
                macro_fn: Variable(
                    "identity",
                ),
                target: Function(
                    "test_func",
                    [],
                    Block(
                        [],
                        Some(
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    ),
                ),
                args: [],
            },
            Declare(
                "result",
                Some(
                    Call(
                        Variable(
                            "test_func",
                        ),
                        [],
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "result",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "identity macro should preserve function",
                        ),
                    ),
                ],
            ),
        ],
    ),
)
```