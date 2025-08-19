# Program
Status: 🟢
Assertions: 1

```rustleaf
fn test_return() {
    return 42;
}

var result = test_return();
assert(result == 42);
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
        1: Token(Ident, "test_return"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Return),
        6: Token(Int, "42"),
        7: Token(Semicolon),
        8: Token(RightBrace),
        9: Token(Var),
        10: Token(Ident, "result"),
        11: Token(Equal),
        12: Token(Ident, "test_return"),
        13: Token(LeftParen),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Ident, "assert"),
        17: Token(LeftParen),
        18: Token(Ident, "result"),
        19: Token(EqualEqual),
        20: Token(Int, "42"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "test_return",
                params: [],
                body: Block {
                    statements: [
                        Return(
                            Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "test_return",
                        ),
                        [],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "result",
                            ),
                            Literal(
                                Int(
                                    42,
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