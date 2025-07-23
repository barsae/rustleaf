# Program 🟢

```rustleaf
[];
```

# Output

```

```

# Result

```rust
Ok(
    List(
        ListRef(
            RefCell {
                value: [],
            },
        ),
    ),
)
```

# Lex

```rust
Ok(
    [
        Token(LeftBracket),
        Token(RightBracket),
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
        [],
        Some(
            List(
                [],
            ),
        ),
    ),
)
```
