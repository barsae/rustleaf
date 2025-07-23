# Program 🔴
```rustleaf
// #[fail_quietly]
item in collection;
```

# Output
```
None
```

# Result
```rust
Err(
    "Undefined variable: collection",
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
        [],
        Some(
            Call(
                GetAttr(
                    Variable(
                        "collection",
                    ),
                    "op_contains",
                ),
                [
                    Variable(
                        "item",
                    ),
                ],
            ),
        ),
    ),
)
```