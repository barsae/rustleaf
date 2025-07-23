# Program

```rustleaf
loop {
    break 42;
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Loop,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Break,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "42",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
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

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Loop { body: Block { statements: [Break(Some(Literal(Int(42))))], final_expr: None } }",
)
```
