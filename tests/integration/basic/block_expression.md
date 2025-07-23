# Program

```rustleaf
var y = {
    var x = 10;
    x + 5
};
assert(y == 15);
```

# Output

```

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
    Block(
        [
            Declare(
                "y",
                Some(
                    Block(
                        [
                            Declare(
                                "x",
                                Some(
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        Some(
                            BinaryOp(
                                Add,
                                Variable(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
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
                    BinaryOp(
                        Eq,
                        Variable(
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
    ),
)
```
