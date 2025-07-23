# Program

```rustleaf
3.14;
```

# Output

```

```

# Result

```rust
Ok(
    Unit,
)
```

# Lex

```rust
Ok(
    [
        Token(Float, "3.14"),
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
                    Float(
                        3.14,
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
            Literal(
                Float(
                    3.14,
                ),
            ),
        ],
        None,
    ),
)
```
