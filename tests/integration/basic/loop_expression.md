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
Err(
    "Expression not yet implemented: Loop { body: Block { statements: [Break(Some(Literal(Int(42))))], final_expr: None } }",
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
Err(
    "Expression not yet implemented: Loop { body: Block { statements: [Break(Some(Literal(Int(42))))], final_expr: None } }",
)
```
