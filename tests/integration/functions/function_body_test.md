# Program
Status: 🟢
Assertions: 1

```rustleaf
fn greet() {
    "hello"
}
assert(greet() == "hello");
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
        0: Token(Fn),
        1: Token(Ident, "greet"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(String, "hello"),
        6: Token(RightBrace),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Ident, "greet"),
        10: Token(LeftParen),
        11: Token(RightParen),
        12: Token(EqualEqual),
        13: Token(String, "hello"),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Eof)
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
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            String(
                                "hello",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "greet",
                                ),
                                [],
                            ),
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    RustValue(<unknown>),
)
```