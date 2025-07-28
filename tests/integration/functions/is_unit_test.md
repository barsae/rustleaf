# Program
Status: 🟢
Assertions: 6

```rustleaf
// Test is_unit with various values
assert(not is_unit(42));
assert(not is_unit("hello"));
assert(not is_unit(true));
assert(not is_unit([]));
assert(not is_unit({}));

// Test with a function that returns Unit
fn side_effect() {
    var x = 1;
}
assert(is_unit(side_effect()));
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(LeftBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "side_effect"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "side_effect"),
        Token(LeftParen),
        Token(RightParen),
        Token(RightParen),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
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
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        String(
                                            "hello",
                                        ),
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        Bool(
                                            true,
                                        ),
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    List(
                                        [],
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Dict(
                                        [],
                                    ),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "side_effect",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "x",
                            ),
                            value: Some(
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        },
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
                                FunctionCall(
                                    Identifier(
                                        "side_effect",
                                    ),
                                    [],
                                ),
                            ],
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
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            Variable(
                                "is_unit",
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
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            Variable(
                                "is_unit",
                            ),
                            [
                                Literal(
                                    String(
                                        "hello",
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            Variable(
                                "is_unit",
                            ),
                            [
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            Variable(
                                "is_unit",
                            ),
                            [
                                List(
                                    [],
                                ),
                            ],
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            Variable(
                                "is_unit",
                            ),
                            [
                                Dict(
                                    [],
                                ),
                            ],
                        ),
                    ),
                ],
            ),
            Function(
                FunctionData {
                    name: "side_effect",
                    params: [],
                    body: Block(
                        [
                            Declare(
                                "x",
                                Some(
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        None,
                    ),
                },
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        Variable(
                            "is_unit",
                        ),
                        [
                            Call(
                                Variable(
                                    "side_effect",
                                ),
                                [],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
)
```