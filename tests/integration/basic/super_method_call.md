# Program 🔴

```rustleaf
super.method();
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
        Token(LeftParen),
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
                MethodCall(
                    Super,
                    "method",
                    [],
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
