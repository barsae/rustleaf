# Program
Status: 🟢
Assertions: 2

```rustleaf
// List indexing and operations
var my_list = [1, 2, 3];
assert(my_list[0] == 1);
assert(my_list[2] == 3);
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
        1: Token(Ident, "my_list"),
        2: Token(Equal),
        3: Token(LeftBracket),
        4: Token(Int, "1"),
        5: Token(Comma),
        6: Token(Int, "2"),
        7: Token(Comma),
        8: Token(Int, "3"),
        9: Token(RightBracket),
        10: Token(Semicolon),
        11: Token(Ident, "assert"),
        12: Token(LeftParen),
        13: Token(Ident, "my_list"),
        14: Token(LeftBracket),
        15: Token(Int, "0"),
        16: Token(RightBracket),
        17: Token(EqualEqual),
        18: Token(Int, "1"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "my_list"),
        24: Token(LeftBracket),
        25: Token(Int, "2"),
        26: Token(RightBracket),
        27: Token(EqualEqual),
        28: Token(Int, "3"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Eof)
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
                    "my_list",
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
                            GetItem(
                                Identifier(
                                    "my_list",
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
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
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