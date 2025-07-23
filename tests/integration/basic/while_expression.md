# Program 🔴

```rustleaf
while x < 10 {
    x = x + 1;
}
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: While { condition: Lt(Identifier(\"x\"), Literal(Int(10))), body: Block { statements: [Assignment { target: Identifier(\"x\"), op: Assign, value: Add(Identifier(\"x\"), Literal(Int(1))) }], final_expr: None } }",
)
```

# Lex

```rust
Ok(
    [
        Token(While),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "10"),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
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
            Expression(
                While {
                    condition: Lt(
                        Identifier(
                            "x",
                        ),
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: Assign,
                                value: Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                },
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: While { condition: Lt(Identifier(\"x\"), Literal(Int(10))), body: Block { statements: [Assignment { target: Identifier(\"x\"), op: Assign, value: Add(Identifier(\"x\"), Literal(Int(1))) }], final_expr: None } }",
)
```
