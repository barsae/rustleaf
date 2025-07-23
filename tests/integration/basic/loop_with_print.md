# Program

```rustleaf
print(loop {
    break 42;
});
```

# Output

```
Int(42)
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
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(Int, "42"),
        Token(Semicolon),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Loop {
                            body: Block {
                                statements: [
                                    Break(
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
            Call(
                Variable(
                    "print",
                ),
                [
                    Loop(
                        Block(
                            [
                                Break(
                                    Some(
                                        Literal(
                                            Int(
                                                42,
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            None,
                        ),
                    ),
                ],
            ),
        ],
        None,
    ),
)
```
