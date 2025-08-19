# Program
Status: 🟢
Assertions: 4

```rustleaf
var t = true;
var f = false;
assert(t == true);
assert(f == false);
assert(t != f);
assert(not f == true);
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
        1: Token(Ident, "t"),
        2: Token(Equal),
        3: Token(True),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "f"),
        7: Token(Equal),
        8: Token(False),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "t"),
        13: Token(EqualEqual),
        14: Token(True),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "f"),
        20: Token(EqualEqual),
        21: Token(False),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "t"),
        27: Token(BangEqual),
        28: Token(Ident, "f"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Not),
        34: Token(Ident, "f"),
        35: Token(EqualEqual),
        36: Token(True),
        37: Token(RightParen),
        38: Token(Semicolon),
        39: Token(Eof)
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
                    "t",
                ),
                value: Some(
                    Literal(
                        Bool(
                            true,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "f",
                ),
                value: Some(
                    Literal(
                        Bool(
                            false,
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
                                "t",
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
                            Identifier(
                                "f",
                            ),
                            Literal(
                                Bool(
                                    false,
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
                        Ne(
                            Identifier(
                                "t",
                            ),
                            Identifier(
                                "f",
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
                            Not(
                                Identifier(
                                    "f",
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