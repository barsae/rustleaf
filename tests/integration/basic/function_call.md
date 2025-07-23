# Program 🔴

```rustleaf
// #[fail_quietly]
func(arg1, arg2);
```

# Output

```

```

# Result

```rust
Err(
    "Undefined variable: func",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "func"),
        Token(LeftParen),
        Token(Ident, "arg1"),
        Token(Comma),
        Token(Ident, "arg2"),
        Token(RightParen),
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
                FunctionCall(
                    Identifier(
                        "func",
                    ),
                    [
                        Identifier(
                            "arg1",
                        ),
                        Identifier(
                            "arg2",
                        ),
                    ],
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
            Call(
                Variable(
                    "func",
                ),
                [
                    Variable(
                        "arg1",
                    ),
                    Variable(
                        "arg2",
                    ),
                ],
            ),
        ),
    ),
)
```
