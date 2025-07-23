# Program

```rustleaf
{
    var x = 10;
    x + 5
}
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
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
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
                ],
                None,
            ),
        ],
        None,
    ),
)
```
