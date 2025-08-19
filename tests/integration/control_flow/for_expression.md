# Program
Status: 🟢
Assertions: 1

```rustleaf
var s = 0;
for x in [1, 2, 3] {
    s += x;
}
assert(s == 6);
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
        1: Token(Ident, "s"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(For),
        6: Token(Ident, "x"),
        7: Token(In),
        8: Token(LeftBracket),
        9: Token(Int, "1"),
        10: Token(Comma),
        11: Token(Int, "2"),
        12: Token(Comma),
        13: Token(Int, "3"),
        14: Token(RightBracket),
        15: Token(LeftBrace),
        16: Token(Ident, "s"),
        17: Token(PlusEqual),
        18: Token(Ident, "x"),
        19: Token(Semicolon),
        20: Token(RightBrace),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "s"),
        24: Token(EqualEqual),
        25: Token(Int, "6"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Eof)
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
                    "s",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "s",
                                ),
                                op: AddAssign,
                                value: Identifier(
                                    "x",
                                ),
                            },
                        ],
                        final_expr: None,
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
                                "s",
                            ),
                            Literal(
                                Int(
                                    6,
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