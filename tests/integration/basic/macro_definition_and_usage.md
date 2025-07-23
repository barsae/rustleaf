# Program

```rustleaf
#[macro]
fn log_calls(ast_node) {
    ast_node
}

#[log_calls]
fn target_function() {
    var x = 42;
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Hash,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Macro,
            text: None,
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Fn,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "log_calls",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "ast_node",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "ast_node",
            ),
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Hash,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "log_calls",
            ),
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Fn,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "target_function",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
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
                "42",
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
            FnDecl {
                name: "target_function",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "x",
                            ),
                            value: Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        },
                    ],
                    final_expr: None,
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
    "Statement not yet implemented: FnDecl { name: \"target_function\", params: [], body: Block { statements: [VarDecl { pattern: Variable(\"x\"), value: Some(Literal(Int(42))) }], final_expr: None }, is_pub: false }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"target_function\", params: [], body: Block { statements: [VarDecl { pattern: Variable(\"x\"), value: Some(Literal(Int(42))) }], final_expr: None }, is_pub: false }",
)
```
