# Program 🔴

```rustleaf
super;
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

# Lex

```rust
Ok(
    [
        Token(Super),
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
