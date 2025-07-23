# Program

```rustleaf
break 42;
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: Break(Some(Literal(Int(42))))",
)
```

# Lex

```rust
Ok(
    [
        Token(Break),
        Token(Int, "42"),
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
                Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
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
        [
            Break(
                Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            ),
        ],
        None,
    ),
)
```
