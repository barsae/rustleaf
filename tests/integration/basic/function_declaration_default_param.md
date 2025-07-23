# Program

```rustleaf
fn greet(name = "world") name
```

# Output

```

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
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "world"),
        Token(RightParen),
        Token(Ident, "name"),
        Token(Eof),
    ],
)
```

# Parse

```rust
Ok(
    Program(
        [
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: Some(
                            String(
                                "world",
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "name",
                        ),
                    ),
                },
                is_pub: false,
            },
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
            Function(
                "greet",
                [
                    "name",
                ],
                Block(
                    [],
                    Some(
                        Variable(
                            "name",
                        ),
                    ),
                ),
            ),
        ],
        None,
    ),
)
```
