# Program

```rustleaf
x = 42;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "42",
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
            Assignment {
                target: Identifier(
                    "x",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        42,
                    ),
                ),
            },
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
            Assign(
                "x",
                Literal(
                    Int(
                        42,
                    ),
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
    "Undefined variable: x",
)
```
