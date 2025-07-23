# Program

```rustleaf
{};
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

# Lex

```rust
Ok(
    [
        Token(LeftBrace),
        Token(RightBrace),
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
