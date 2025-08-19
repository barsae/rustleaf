# Program
Status: 🟢
Assertions: 3

```rustleaf
var list = [1, 2, 3, "hello", true];
assert(list[0] == 1);
assert(list[3] == "hello");
assert(list[4] == true);
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
        1: Token(Ident, "list"),
        2: Token(Equal),
        3: Token(LeftBracket),
        4: Token(Int, "1"),
        5: Token(Comma),
        6: Token(Int, "2"),
        7: Token(Comma),
        8: Token(Int, "3"),
        9: Token(Comma),
        10: Token(String, "hello"),
        11: Token(Comma),
        12: Token(True),
        13: Token(RightBracket),
        14: Token(Semicolon),
        15: Token(Ident, "assert"),
        16: Token(LeftParen),
        17: Token(Ident, "list"),
        18: Token(LeftBracket),
        19: Token(Int, "0"),
        20: Token(RightBracket),
        21: Token(EqualEqual),
        22: Token(Int, "1"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Ident, "assert"),
        26: Token(LeftParen),
        27: Token(Ident, "list"),
        28: Token(LeftBracket),
        29: Token(Int, "3"),
        30: Token(RightBracket),
        31: Token(EqualEqual),
        32: Token(String, "hello"),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Ident, "list"),
        38: Token(LeftBracket),
        39: Token(Int, "4"),
        40: Token(RightBracket),
        41: Token(EqualEqual),
        42: Token(True),
        43: Token(RightParen),
        44: Token(Semicolon),
        45: Token(Eof)
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
                    "list",
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
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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