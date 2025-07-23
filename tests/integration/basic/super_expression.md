# Program

```rustleaf
super;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Super,
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
            Expression(
                Super,
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: Super",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Super",
)
```
