# Program

```rustleaf
return;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Return,
            text: None,
        },
        Token {
            token_type: Semicolon,
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
            Return(
                None,
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
            Return(
                None,
            ),
        ],
        None,
    ),
)
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: Return(None)",
)
```
