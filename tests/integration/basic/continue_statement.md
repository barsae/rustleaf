# Program

```rustleaf
continue;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Continue,
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
            Continue,
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
            Continue,
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
    "eval not implemented for: Continue",
)
```
