# Program
Status: 🟢
Assertions: 3

```rustleaf
var y;
var z;
y = 100;
z = "assigned later";
assert(y == 100);
assert(z == "assigned later");
assert(y + 23 == 123);
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
        1: Token(Ident, "y"),
        2: Token(Semicolon),
        3: Token(Var),
        4: Token(Ident, "z"),
        5: Token(Semicolon),
        6: Token(Ident, "y"),
        7: Token(Equal),
        8: Token(Int, "100"),
        9: Token(Semicolon),
        10: Token(Ident, "z"),
        11: Token(Equal),
        12: Token(String, "assigned later"),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Ident, "y"),
        17: Token(EqualEqual),
        18: Token(Int, "100"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "z"),
        24: Token(EqualEqual),
        25: Token(String, "assigned later"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "y"),
        31: Token(Plus),
        32: Token(Int, "23"),
        33: Token(EqualEqual),
        34: Token(Int, "123"),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Eof)
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
                    "y",
                ),
                value: None,
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: None,
            },
            Assignment {
                target: Identifier(
                    "y",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        100,
                    ),
                ),
            },
            Assignment {
                target: Identifier(
                    "z",
                ),
                op: Assign,
                value: Literal(
                    String(
                        "assigned later",
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
                                "y",
                            ),
                            Literal(
                                Int(
                                    100,
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
                                String(
                                    "assigned later",
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
                                    "y",
                                ),
                                Literal(
                                    Int(
                                        23,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    123,
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