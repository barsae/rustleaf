# Program

```rustleaf
loop {
    break 42;
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
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(Int, "42"),
        Token(Semicolon),
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
        None,
    ),
)
```
