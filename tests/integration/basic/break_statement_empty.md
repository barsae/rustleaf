# Program

```rustleaf
break;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Break,
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
            Break(
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
            Break(
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
    "eval not implemented for: Break(None)",
)
```
