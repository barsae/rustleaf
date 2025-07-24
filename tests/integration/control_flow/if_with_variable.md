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
    Block(
        [
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            If(
                                Call(
                                    GetAttr(
                                        Variable(
                                            "x",
                                        ),
                                        "op_gt",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                0,
                                            ),
                                        ),
                                    ],
                                ),
                                Block(
                                    [],
                                    Some(
                                        Literal(
                                            String(
                                                "positive",
                                            ),
                                        ),
                                    ),
                                ),
                                Some(
                                    Block(
                                        [],
                                        Some(
                                            Literal(
                                                String(
                                                    "zero or negative",
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "positive",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```