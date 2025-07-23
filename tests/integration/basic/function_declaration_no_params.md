# Program

```rustleaf
fn hello() 42
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"hello\", params: [], body: Block { statements: [], final_expr: Some(Literal(Int(42))) }, is_pub: false }",
)
```

# Lex

```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "hello"),
        Token(LeftParen),
        Token(RightParen),
        Token(Int, "42"),
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
                name: "hello",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            Int(
                                42,
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
    "Statement not yet implemented: FnDecl { name: \"hello\", params: [], body: Block { statements: [], final_expr: Some(Literal(Int(42))) }, is_pub: false }",
)
```
