# Program 🔴

```rustleaf
// #[fail_quietly]
"Value: ${x + y * 2}";
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: InterpolatedString([Text(\"Value: \"), Expression(Add(Identifier(\"x\"), Mul(Identifier(\"y\"), Literal(Int(2)))))])",
)
```

# Lex

```rust
Ok(
    [
        Token(StringPart, "Value: "),
        Token(InterpolationStart),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(Star),
        Token(Int, "2"),
        Token(InterpolationEnd),
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
                InterpolatedString(
                    [
                        Text(
                            "Value: ",
                        ),
                        Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Mul(
                                    Identifier(
                                        "y",
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ),
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
Err(
    "Expression not yet implemented: InterpolatedString([Text(\"Value: \"), Expression(Add(Identifier(\"x\"), Mul(Identifier(\"y\"), Literal(Int(2)))))])",
)
```
