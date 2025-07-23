# Program

```rustleaf
break;
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

# Lex

```rust
Ok(
    [
        Token(Break),
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
