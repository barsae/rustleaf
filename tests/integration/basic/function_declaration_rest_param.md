# Program 🟢
```rustleaf
fn sum(*args) args
```

# Output
```
None
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
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(Star),
        Token(Ident, "args"),
        Token(RightParen),
        Token(Ident, "args"),
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
                name: "sum",
                params: [
                    Parameter {
                        name: "args",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "args",
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
                "sum",
                [
                    "args",
                ],
                Block(
                    [],
                    Some(
                        Variable(
                            "args",
                        ),
                    ),
                ),
            ),
        ],
        None,
    ),
)
```