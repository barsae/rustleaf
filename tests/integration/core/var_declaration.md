# Program
Status: 🟢
Assertions: 4

```rustleaf
var x = 42;
var y = "hello";
var z = true;
assert(x == 42);
assert(y == "hello");
assert(z == true);
assert(x + 8 == 50);
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
        3: Token(Int, "42"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "y"),
        7: Token(Equal),
        8: Token(String, "hello"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "z"),
        12: Token(Equal),
        13: Token(True),
        14: Token(Semicolon),
        15: Token(Ident, "assert"),
        16: Token(LeftParen),
        17: Token(Ident, "x"),
        18: Token(EqualEqual),
        19: Token(Int, "42"),
        20: Token(RightParen),
        21: Token(Semicolon),
        22: Token(Ident, "assert"),
        23: Token(LeftParen),
        24: Token(Ident, "y"),
        25: Token(EqualEqual),
        26: Token(String, "hello"),
        27: Token(RightParen),
        28: Token(Semicolon),
        29: Token(Ident, "assert"),
        30: Token(LeftParen),
        31: Token(Ident, "z"),
        32: Token(EqualEqual),
        33: Token(True),
        34: Token(RightParen),
        35: Token(Semicolon),
        36: Token(Ident, "assert"),
        37: Token(LeftParen),
        38: Token(Ident, "x"),
        39: Token(Plus),
        40: Token(Int, "8"),
        41: Token(EqualEqual),
        42: Token(Int, "50"),
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "y",
                ),
                value: Some(
                    Literal(
                        String(
                            "hello",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: Some(
                    Literal(
                        Bool(
                            true,
                        ),
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
                                "x",
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
                            Identifier(
                                "z",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        8,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    50,
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