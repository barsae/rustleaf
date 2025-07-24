# Program
Status: 🟢
Assertions: 0

```rustleaf
fn hello() { 42 }
```

# Output
None

# Result
```rust
Ok(
    Unit,
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
Ok(
    Block(
        [
            Function(
                "hello",
                [],
                Block(
                    [],
                    Some(
                        Literal(
                            Int(
                                42,
                            ),
                        ),
                    ),
                ),
            ),
        ],
        None,
    ),
)
```