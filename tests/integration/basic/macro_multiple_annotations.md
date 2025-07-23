# Program

```rustleaf
#[first_macro]
#[second_macro(config: "test")]
#[third_macro]
fn decorated_function() {
    var result = 42;
}
```

# Output

```

```

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
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "first_macro"),
        Token(RightBracket),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "second_macro"),
        Token(LeftParen),
        Token(Ident, "config"),
        Token(Colon),
        Token(String, "test"),
        Token(RightParen),
        Token(RightBracket),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "third_macro"),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "decorated_function"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "result"),
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
                name: "decorated_function",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "result",
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
Ok(
    Block(
        [
            Function(
                "decorated_function",
                [],
                Block(
                    [
                        Declare(
                            "result",
                            Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        ),
                    ],
                    None,
                ),
            ),
        ],
        None,
    ),
)
```
