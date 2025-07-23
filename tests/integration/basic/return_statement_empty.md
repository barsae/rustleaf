# Program

```rustleaf
return;
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

# Lex

```rust
Ok(
    [
        Token(Return),
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
