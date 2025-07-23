# Program

```rustleaf
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
