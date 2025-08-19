# Program
Status: 🟢
Assertions: 1

```rustleaf
var i;
try {
    raise(42);
} catch e {
    i = e;
}
assert(i == 42);
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
        1: Token(Ident, "i"),
        2: Token(Semicolon),
        3: Token(Try),
        4: Token(LeftBrace),
        5: Token(Ident, "raise"),
        6: Token(LeftParen),
        7: Token(Int, "42"),
        8: Token(RightParen),
        9: Token(Semicolon),
        10: Token(RightBrace),
        11: Token(Catch),
        12: Token(Ident, "e"),
        13: Token(LeftBrace),
        14: Token(Ident, "i"),
        15: Token(Equal),
        16: Token(Ident, "e"),
        17: Token(Semicolon),
        18: Token(RightBrace),
        19: Token(Ident, "assert"),
        20: Token(LeftParen),
        21: Token(Ident, "i"),
        22: Token(EqualEqual),
        23: Token(Int, "42"),
        24: Token(RightParen),
        25: Token(Semicolon),
        26: Token(Eof)
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
                    "i",
                ),
                value: None,
            },
            Expression(
                Try {
                    body: Block {
                        statements: [
                            Expression(
                                FunctionCall(
                                    Identifier(
                                        "raise",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                42,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        ],
                        final_expr: None,
                    },
                    catch: CatchClause {
                        pattern: Variable(
                            "e",
                        ),
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "i",
                                    ),
                                    op: Assign,
                                    value: Identifier(
                                        "e",
                                    ),
                                },
                            ],
                            final_expr: None,
                        },
                    },
                },
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "i",
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