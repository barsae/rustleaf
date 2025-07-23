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

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"target_function\", params: [], body: Block { statements: [VarDecl { pattern: Variable(\"x\"), value: Some(Literal(Int(42))) }], final_expr: None }, is_pub: false }",
)
```

# Lex

```rust
Ok(
    [
        Token(Hash),
        Token(LeftBracket),
        Token(Macro),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "log_calls"),
        Token(LeftParen),
        Token(Ident, "ast_node"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "ast_node"),
        Token(RightBrace),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "log_calls"),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "target_function"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
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
