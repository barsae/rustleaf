# Program

```rustleaf
true;
```

# Output

```

```

# Result

```rust
Ok(
    Bool(
        true,
    ),
)
```

# Lex

```rust
Ok(
    [
        Token(True),
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
                Literal(
                    Bool(
                        true,
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
        [],
        Some(
            Literal(
                Bool(
                    true,
                ),
            ),
        ),
    ),
)
```
