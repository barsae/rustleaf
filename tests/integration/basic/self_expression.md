# Program

```rustleaf
self.field;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "self",
            ),
        },
        Token {
            token_type: Dot,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "field",
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
        [
            GetAttr(
                Variable(
                    "self",
                ),
                "field",
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
    "eval not implemented for: GetAttr(Variable(\"self\"), \"field\")",
)
```
