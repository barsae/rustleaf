# Program

```rustleaf
a is b;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "a",
            ),
        },
        Token {
            token_type: Is,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "b",
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
                Is(
                    Identifier(
                        "a",
                    ),
                    Identifier(
                        "b",
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
                Is,
                Variable(
                    "a",
                ),
                Variable(
                    "b",
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
    "eval not implemented for: BinaryOp(Is, Variable(\"a\"), Variable(\"b\"))",
)
```
