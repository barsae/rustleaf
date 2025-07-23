# Program

```rustleaf
var x = 1..2;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Var,
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
            token_type: Int,
            text: Some(
                "1",
            ),
        },
        Token {
            token_type: DotDot,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "2",
            ),
        },
        Token {
            token_type: Semicolon,
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
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        Literal(
                            Int(
                                2,
                            ),
                        ),
                    ),
                ),
            },
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(2)))",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(2)))",
)
```
