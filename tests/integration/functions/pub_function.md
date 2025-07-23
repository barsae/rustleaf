# Program
Status: 🟢

```rustleaf
pub fn greet(name) {
    print("Hello, " + name);
}
```

# Output
None

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
        Token(Pub),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(String, "Hello, "),
        Token(Plus),
        Token(Ident, "name"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
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
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        Expression(
                            FunctionCall(
                                Identifier(
                                    "print",
                                ),
                                [
                                    Add(
                                        Literal(
                                            String(
                                                "Hello, ",
                                            ),
                                        ),
                                        Identifier(
                                            "name",
                                        ),
                                    ),
                                ],
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: true,
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
                    [
                        Call(
                            Variable(
                                "print",
                            ),
                            [
                                Call(
                                    GetAttr(
                                        Literal(
                                            String(
                                                "Hello, ",
                                            ),
                                        ),
                                        "op_add",
                                    ),
                                    [
                                        Variable(
                                            "name",
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                    None,
                ),
            ),
        ],
        None,
    ),
)
```