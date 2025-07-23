# Program 🟢

```rustleaf
42;
```

# Output

```

```

# Result

```rust
Ok(
    Int(
        42,
    ),
)
```

# Lex

```rust
Ok(
    [
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
            Expression(
                Literal(
                    Int(
                        42,
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
                Int(
                    42,
                ),
            ),
        ),
    ),
)
```
