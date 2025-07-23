# Program

```rustleaf
raise("File not found");
```

# Output

```

```

# Result

```rust
Err(
    "Undefined variable: raise",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "raise"),
        Token(LeftParen),
        Token(String, "File not found"),
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
                        "raise",
                    ),
                    [
                        Literal(
                            String(
                                "File not found",
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
        [],
        Some(
            Call(
                Variable(
                    "raise",
                ),
                [
                    Literal(
                        String(
                            "File not found",
                        ),
                    ),
                ],
            ),
        ),
    ),
)
```
