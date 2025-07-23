# Program 🟢
```rustleaf
print("hello, world");
```

# Output
```
String("hello, world")
```

# Result
```rust
Ok(
    Unit,
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "print"),
        Token(LeftParen),
        Token(String, "hello, world"),
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
                        "print",
                    ),
                    [
                        Literal(
                            String(
                                "hello, world",
                            ),
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
                    "print",
                ),
                [
                    Literal(
                        String(
                            "hello, world",
                        ),
                    ),
                ],
            ),
        ),
    ),
)
```