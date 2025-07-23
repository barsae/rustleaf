# Program

```rustleaf
if true { 42 }
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

# Lex

```rust
Ok(
    [
        Token(If),
        Token(True),
        Token(LeftBrace),
        Token(Int, "42"),
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
