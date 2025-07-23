# Program 🟢
```rustleaf
fn add(x, y) x + y
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
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
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
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
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
                "add",
                [
                    "x",
                    "y",
                ],
                Block(
                    [],
                    Some(
                        Call(
                            GetAttr(
                                Variable(
                                    "x",
                                ),
                                "op_add",
                            ),
                            [
                                Variable(
                                    "y",
                                ),
                            ],
                        ),
                    ),
                ),
            ),
        ],
        None,
    ),
)
```