# Program

```rustleaf
if x > 0 {
    "positive"
} else {
    "zero or negative"
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: If,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Greater,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "0",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "positive",
            ),
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Else,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "zero or negative",
            ),
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
                If {
                    condition: Gt(
                        Identifier(
                            "x",
                        ),
                        Literal(
                            Int(
                                0,
                            ),
                        ),
                    ),
                    then_expr: Block {
                        statements: [],
                        final_expr: Some(
                            Literal(
                                String(
                                    "positive",
                                ),
                            ),
                        ),
                    },
                    else_expr: Some(
                        Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "zero or negative",
                                    ),
                                ),
                            ),
                        },
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
    "Expression not yet implemented: If { condition: Gt(Identifier(\"x\"), Literal(Int(0))), then_expr: Block { statements: [], final_expr: Some(Literal(String(\"positive\"))) }, else_expr: Some(Block { statements: [], final_expr: Some(Literal(String(\"zero or negative\"))) }) }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: If { condition: Gt(Identifier(\"x\"), Literal(Int(0))), then_expr: Block { statements: [], final_expr: Some(Literal(String(\"positive\"))) }, else_expr: Some(Block { statements: [], final_expr: Some(Literal(String(\"zero or negative\"))) }) }",
)
```
