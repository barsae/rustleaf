# Program

```rustleaf
fn sum(*args) args
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
                "sum",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Star,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "args",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "args",
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
                name: "sum",
                params: [
                    Parameter {
                        name: "args",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "args",
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
    "Statement not yet implemented: FnDecl { name: \"sum\", params: [Parameter { name: \"args\", default: None, kind: Rest }], body: Block { statements: [], final_expr: Some(Identifier(\"args\")) }, is_pub: false }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"sum\", params: [Parameter { name: \"args\", default: None, kind: Rest }], body: Block { statements: [], final_expr: Some(Identifier(\"args\")) }, is_pub: false }",
)
```
