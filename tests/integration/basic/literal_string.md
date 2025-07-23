# Program

```rustleaf
"hello";
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
        Token(String, "hello"),
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
                    String(
                        "hello",
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
                String(
                    "hello",
                ),
            ),
        ],
        None,
    ),
)
```
