# Program
Status: 🟢
Assertions: 3

```rustleaf
var [a, b, c] = [1, 2, 3];
assert(a == 1);
assert(b == 2);
assert(c == 3);
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
        2: Token(Ident, "a"),
        3: Token(Comma),
        4: Token(Ident, "b"),
        5: Token(Comma),
        6: Token(Ident, "c"),
        7: Token(RightBracket),
        8: Token(Equal),
        9: Token(LeftBracket),
        10: Token(Int, "1"),
        11: Token(Comma),
        12: Token(Int, "2"),
        13: Token(Comma),
        14: Token(Int, "3"),
        15: Token(RightBracket),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "a"),
        20: Token(EqualEqual),
        21: Token(Int, "1"),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "b"),
        27: Token(EqualEqual),
        28: Token(Int, "2"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Ident, "c"),
        34: Token(EqualEqual),
        35: Token(Int, "3"),
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
                pattern: List(
                    [
                        Variable(
                            "a",
                        ),
                        Variable(
                            "b",
                        ),
                        Variable(
                            "c",
                        ),
                    ],
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
                                "a",
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
                                "b",
                            ),
                            Literal(
                                Int(
                                    2,
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
                                "c",
                            ),
                            Literal(
                                Int(
                                    3,
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