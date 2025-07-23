# Program

```rustleaf
fn greet(name = "world") name
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Fn,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "greet",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "world",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
            ),
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
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: Some(
                            String(
                                "world",
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "name",
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
Err(
    "Statement not yet implemented: FnDecl { name: \"greet\", params: [Parameter { name: \"name\", default: Some(String(\"world\")), kind: Regular }], body: Block { statements: [], final_expr: Some(Identifier(\"name\")) }, is_pub: false }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"greet\", params: [Parameter { name: \"name\", default: Some(String(\"world\")), kind: Regular }], body: Block { statements: [], final_expr: Some(Identifier(\"name\")) }, is_pub: false }",
)
```
