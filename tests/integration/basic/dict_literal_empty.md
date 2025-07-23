# Program 🟢

```rustleaf
{};
```

# Output

```

```

# Result

```rust
Ok(
    Dict(
        DictRef(
            RefCell {
                value: {},
            },
        ),
    ),
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
        [],
        Some(
            Dict(
                [],
            ),
        ),
    ),
)
```
