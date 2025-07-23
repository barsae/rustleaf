# Program 🔴

```rustleaf
|x| x + 1;
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Lambda { params: [\"x\"], body: Expression(Add(Identifier(\"x\"), Literal(Int(1)))) }",
)
```

# Lex

```rust
Ok(
    [
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
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
                Lambda {
                    params: [
                        "x",
                    ],
                    body: Expression(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: Lambda { params: [\"x\"], body: Expression(Add(Identifier(\"x\"), Literal(Int(1)))) }",
)
```
