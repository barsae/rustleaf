# Program

```rustleaf
obj.field = 10;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "obj",
            ),
        },
        Token {
            token_type: Dot,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "field",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "10",
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
            Assignment {
                target: GetAttr(
                    Identifier(
                        "obj",
                    ),
                    "field",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        10,
                    ),
                ),
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
            SetAttr(
                Variable(
                    "obj",
                ),
                "field",
                Literal(
                    Int(
                        10,
                    ),
                ),
            ),
        ],
        None,
    ),
)
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: SetAttr(Variable(\"obj\"), \"field\", Literal(Int(10)))",
)
```
