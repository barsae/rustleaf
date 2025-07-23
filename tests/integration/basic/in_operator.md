# Program

```rustleaf
item in collection;
```

# Output

```

```

# Result

```rust
Err(
    "Undefined variable: item",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "item"),
        Token(In),
        Token(Ident, "collection"),
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
                In(
                    Identifier(
                        "item",
                    ),
                    Identifier(
                        "collection",
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
            BinaryOp(
                In,
                Variable(
                    "item",
                ),
                Variable(
                    "collection",
                ),
            ),
        ],
        None,
    ),
)
```
