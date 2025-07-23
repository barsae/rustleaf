# Program

```rustleaf
var x = 5;
print(if x > 0 { "positive" } else { "zero or negative" });
```

# Output

```

```

# Result

```rust
Err(
    "No attribute 'op_gt' on value Int(5)",
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
        Token(Ident, "print"),
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
                        "print",
                    ),
                    [
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
                    "print",
                ),
                [
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
                ],
            ),
        ),
    ),
)
```
