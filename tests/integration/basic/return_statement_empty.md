# Program 🔴

```rustleaf
return;
```

# Output

```

```

# Result

```rust
Err(
    "Unexpected return: Unit",
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
