# Program

```rustleaf
fn add(x, y) x + y
print(add(2, 3));
```

# Output

```
Int(5)
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
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        FunctionCall(
                            Identifier(
                                "add",
                            ),
                            [
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                            ],
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
        Some(
            Call(
                Variable(
                    "print",
                ),
                [
                    Call(
                        Variable(
                            "add",
                        ),
                        [
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```
