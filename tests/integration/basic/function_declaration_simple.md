# Program

```rustleaf
fn add(x, y) x + y
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
                "add",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "y",
            ),
        },
        Token {
            token_type: RightParen,
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
            token_type: Ident,
            text: Some(
                "y",
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
        ],
    ),
)
```

# Eval

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"add\", params: [Parameter { name: \"x\", default: None, kind: Regular }, Parameter { name: \"y\", default: None, kind: Regular }], body: Block { statements: [], final_expr: Some(Add(Identifier(\"x\"), Identifier(\"y\"))) }, is_pub: false }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"add\", params: [Parameter { name: \"x\", default: None, kind: Regular }, Parameter { name: \"y\", default: None, kind: Regular }], body: Block { statements: [], final_expr: Some(Add(Identifier(\"x\"), Identifier(\"y\"))) }, is_pub: false }",
)
```
