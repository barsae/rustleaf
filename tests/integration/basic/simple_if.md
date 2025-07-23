# Program

```rustleaf
if true { 42 }
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
            token_type: True,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "42",
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
                    condition: Literal(
                        Bool(
                            true,
                        ),
                    ),
                    then_expr: Block {
                        statements: [],
                        final_expr: Some(
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    },
                    else_expr: None,
                },
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: If { condition: Literal(Bool(true)), then_expr: Block { statements: [], final_expr: Some(Literal(Int(42))) }, else_expr: None }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: If { condition: Literal(Bool(true)), then_expr: Block { statements: [], final_expr: Some(Literal(Int(42))) }, else_expr: None }",
)
```
