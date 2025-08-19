# Program
Status: 🟢
Assertions: 6

```rustleaf
// Test is_unit with various values
assert(not is_unit(42));
assert(not is_unit("hello"));
assert(not is_unit(true));
assert(not is_unit([]));
assert(not is_unit({}));

// Test with a function that returns Unit
fn side_effect() {
    var x = 1;
}
assert(is_unit(side_effect()));
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(Not),
        3: Token(Ident, "is_unit"),
        4: Token(LeftParen),
        5: Token(Int, "42"),
        6: Token(RightParen),
        7: Token(RightParen),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Not),
        12: Token(Ident, "is_unit"),
        13: Token(LeftParen),
        14: Token(String, "hello"),
        15: Token(RightParen),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Not),
        21: Token(Ident, "is_unit"),
        22: Token(LeftParen),
        23: Token(True),
        24: Token(RightParen),
        25: Token(RightParen),
        26: Token(Semicolon),
        27: Token(Ident, "assert"),
        28: Token(LeftParen),
        29: Token(Not),
        30: Token(Ident, "is_unit"),
        31: Token(LeftParen),
        32: Token(LeftBracket),
        33: Token(RightBracket),
        34: Token(RightParen),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Ident, "assert"),
        38: Token(LeftParen),
        39: Token(Not),
        40: Token(Ident, "is_unit"),
        41: Token(LeftParen),
        42: Token(LeftBrace),
        43: Token(RightBrace),
        44: Token(RightParen),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Fn),
        48: Token(Ident, "side_effect"),
        49: Token(LeftParen),
        50: Token(RightParen),
        51: Token(LeftBrace),
        52: Token(Var),
        53: Token(Ident, "x"),
        54: Token(Equal),
        55: Token(Int, "1"),
        56: Token(Semicolon),
        57: Token(RightBrace),
        58: Token(Ident, "assert"),
        59: Token(LeftParen),
        60: Token(Ident, "is_unit"),
        61: Token(LeftParen),
        62: Token(Ident, "side_effect"),
        63: Token(LeftParen),
        64: Token(RightParen),
        65: Token(RightParen),
        66: Token(RightParen),
        67: Token(Semicolon),
        68: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
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
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        String(
                                            "hello",
                                        ),
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        Bool(
                                            true,
                                        ),
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    List(
                                        [],
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Dict(
                                        [],
                                    ),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "side_effect",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "x",
                            ),
                            value: Some(
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        },
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
                                FunctionCall(
                                    Identifier(
                                        "side_effect",
                                    ),
                                    [],
                                ),
                            ],
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