# Program
Status: 🟢
Assertions: 1

```rustleaf
var y = {
    var x = 10;
    x + 5
};
assert(y == 15);
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
        0: Token(Var),
        1: Token(Ident, "y"),
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(Var),
        5: Token(Ident, "x"),
        6: Token(Equal),
        7: Token(Int, "10"),
        8: Token(Semicolon),
        9: Token(Ident, "x"),
        10: Token(Plus),
        11: Token(Int, "5"),
        12: Token(RightBrace),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Ident, "y"),
        17: Token(EqualEqual),
        18: Token(Int, "15"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: Variable(
                    "y",
                ),
                value: Some(
                    Block(
                        Block {
                            statements: [
                                VarDecl {
                                    pattern: Variable(
                                        "x",
                                    ),
                                    value: Some(
                                        Literal(
                                            Int(
                                                10,
                                            ),
                                        ),
                                    ),
                                },
                            ],
                            final_expr: Some(
                                Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                        },
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
                                "y",
                            ),
                            Literal(
                                Int(
                                    15,
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