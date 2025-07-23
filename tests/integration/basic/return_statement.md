# Program

```rustleaf
return 42;
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: Return(Some(Literal(Int(42))))",
)
```

# Lex

```rust
Ok(
    [
        Token(Return),
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
            Return(
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
            Return(
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
