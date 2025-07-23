# Program 🟢
```rustleaf
var x = 1;
{
    var x = 2;
    print(x);
}
print(x);
```

# Output
```
Int(2)
Int(1)
```

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
        Token(Int, "1"),
        Token(Semicolon),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "x"),
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
                            1,
                        ),
                    ),
                ),
            },
            Expression(
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
                                            2,
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
                                        Identifier(
                                            "x",
                                        ),
                                    ],
                                ),
                            ),
                        ],
                        final_expr: None,
                    },
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Identifier(
                            "x",
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
                            1,
                        ),
                    ),
                ),
            ),
            Block(
                [
                    Declare(
                        "x",
                        Some(
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                        ),
                    ),
                    Call(
                        Variable(
                            "print",
                        ),
                        [
                            Variable(
                                "x",
                            ),
                        ],
                    ),
                ],
                None,
            ),
        ],
        Some(
            Call(
                Variable(
                    "print",
                ),
                [
                    Variable(
                        "x",
                    ),
                ],
            ),
        ),
    ),
)
```