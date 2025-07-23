# Program

```rustleaf
continue;
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

# Lex

```rust
Ok(
    [
        Token(Continue),
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
