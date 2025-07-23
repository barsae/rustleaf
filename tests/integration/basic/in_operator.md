# Program

```rustleaf
item in collection;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "item",
            ),
        },
        Token {
            token_type: In,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "collection",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
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

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: BinaryOp(In, Variable(\"item\"), Variable(\"collection\"))",
)
```
