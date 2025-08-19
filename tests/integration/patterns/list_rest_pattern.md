# Program
Status: 🟢
Assertions: 2

```rustleaf
var [first, *rest] = [1, 2, 3, 4];
assert(first == 1);
assert(rest == [2, 3, 4]);
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
        1: Token(LeftBracket),
        2: Token(Ident, "first"),
        3: Token(Comma),
        4: Token(Star),
        5: Token(Ident, "rest"),
        6: Token(RightBracket),
        7: Token(Equal),
        8: Token(LeftBracket),
        9: Token(Int, "1"),
        10: Token(Comma),
        11: Token(Int, "2"),
        12: Token(Comma),
        13: Token(Int, "3"),
        14: Token(Comma),
        15: Token(Int, "4"),
        16: Token(RightBracket),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Ident, "first"),
        21: Token(EqualEqual),
        22: Token(Int, "1"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Ident, "assert"),
        26: Token(LeftParen),
        27: Token(Ident, "rest"),
        28: Token(EqualEqual),
        29: Token(LeftBracket),
        30: Token(Int, "2"),
        31: Token(Comma),
        32: Token(Int, "3"),
        33: Token(Comma),
        34: Token(Int, "4"),
        35: Token(RightBracket),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: ListRest(
                    [
                        Variable(
                            "first",
                        ),
                    ],
                    Some(
                        "rest",
                    ),
                ),
                value: Some(
                    List(
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
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                        ],
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
                                "first",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "rest",
                            ),
                            List(
                                [
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
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
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