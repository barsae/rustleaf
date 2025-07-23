# Program

```rustleaf
1 + 2;
```

# Output

```

```

# Result

```rust
Ok(
    Int(
        3,
    ),
)
```

# Lex

```rust
Ok(
    [
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
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
                Add(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                    Literal(
                        Int(
                            2,
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
        [],
        Some(
            Call(
                GetAttr(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                    "op_add",
                ),
                [
                    Literal(
                        Int(
                            2,
                        ),
                    ),
                ],
            ),
        ),
    ),
)
```
