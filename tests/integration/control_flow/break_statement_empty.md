# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 0;
loop {
    x += 1;
    break;
    x += 1;
}
assert(x == 1);
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Loop),
        6: Token(LeftBrace),
        7: Token(Ident, "x"),
        8: Token(PlusEqual),
        9: Token(Int, "1"),
        10: Token(Semicolon),
        11: Token(Break),
        12: Token(Semicolon),
        13: Token(Ident, "x"),
        14: Token(PlusEqual),
        15: Token(Int, "1"),
        16: Token(Semicolon),
        17: Token(RightBrace),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Ident, "x"),
        21: Token(EqualEqual),
        22: Token(Int, "1"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Eof)
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
                    "x",
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
                Loop {
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            },
                            Break(
                                None,
                            ),
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
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
                                "x",
                            ),
                            Literal(
                                Int(
                                    1,
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