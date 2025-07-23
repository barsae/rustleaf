# Program

```rustleaf
123;
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
        Token(Int, "123"),
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
                        123,
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
                    123,
                ),
            ),
        ],
        None,
    ),
)
```
