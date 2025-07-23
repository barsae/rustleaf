# Program

```rustleaf
42;
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
        [
            Literal(
                Int(
                    42,
                ),
            ),
        ],
        None,
    ),
)
```
