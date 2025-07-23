# Program

```rustleaf
[];
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: List([])",
)
```

# Lex

```rust
Ok(
    [
        Token(LeftBracket),
        Token(RightBracket),
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
                List(
                    [],
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
            List(
                [],
            ),
        ),
    ),
)
```
