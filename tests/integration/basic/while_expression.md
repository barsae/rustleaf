# Program

```rustleaf
while x < 10 {
    x = x + 1;
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: While,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Less,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "10",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Plus,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "1",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
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

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: While { condition: Lt(Identifier(\"x\"), Literal(Int(10))), body: Block { statements: [Assignment { target: Identifier(\"x\"), op: Assign, value: Add(Identifier(\"x\"), Literal(Int(1))) }], final_expr: None } }",
)
```
