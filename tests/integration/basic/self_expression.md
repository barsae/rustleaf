# Program

```rustleaf
self.field;
```

# Output

```

```

# Result

```rust
Err(
    "Undefined variable: self",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "field"),
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
                GetAttr(
                    Identifier(
                        "self",
                    ),
                    "field",
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
            GetAttr(
                Variable(
                    "self",
                ),
                "field",
            ),
        ),
    ),
)
```
