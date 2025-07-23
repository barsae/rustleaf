# Program

```rustleaf
[];
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: RightBracket,
            text: None,
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
                List(
                    [],
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
            List(
                [],
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
    "eval not implemented for: List([])",
)
```
