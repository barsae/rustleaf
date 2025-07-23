# Program 🔴

```rustleaf
// #[fail_quietly]
super.method;
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Super",
)
```

# Lex

```rust
Ok(
    [
        Token(Super),
        Token(Dot),
        Token(Ident, "method"),
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
                GetAttr(
                    Super,
                    "method",
                ),
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: Super",
)
```
