# Program

```rustleaf
{};
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: RightBrace,
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
                Dict(
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
            Dict(
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
    "eval not implemented for: Dict([])",
)
```
