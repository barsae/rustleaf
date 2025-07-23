# Program 🔴

```rustleaf
// #[fail_quietly]
a is b;
```

# Output

```

```

# Result

```rust
Err(
    "Undefined variable: a",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "a"),
        Token(Is),
        Token(Ident, "b"),
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
        [],
        Some(
            Is(
                Variable(
                    "a",
                ),
                Variable(
                    "b",
                ),
            ),
        ),
    ),
)
```
