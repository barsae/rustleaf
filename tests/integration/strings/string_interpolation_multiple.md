# Program
Status: 🟢
Assertions: 1

```rustleaf
var a = 3;
var b = 7;
var result = "${a} and ${b} equals ${a + b}";
assert(result == "3 and 7 equals 10");
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
        1: Token(Ident, "a"),
        2: Token(Equal),
        3: Token(Int, "3"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "b"),
        7: Token(Equal),
        8: Token(Int, "7"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "result"),
        12: Token(Equal),
        13: Token(InterpolationStart),
        14: Token(Ident, "a"),
        15: Token(InterpolationEnd),
        16: Token(StringPart, " and "),
        17: Token(InterpolationStart),
        18: Token(Ident, "b"),
        19: Token(InterpolationEnd),
        20: Token(StringPart, " equals "),
        21: Token(InterpolationStart),
        22: Token(Ident, "a"),
        23: Token(Plus),
        24: Token(Ident, "b"),
        25: Token(InterpolationEnd),
        26: Token(Semicolon),
        27: Token(Ident, "assert"),
        28: Token(LeftParen),
        29: Token(Ident, "result"),
        30: Token(EqualEqual),
        31: Token(String, "3 and 7 equals 10"),
        32: Token(RightParen),
        33: Token(Semicolon),
        34: Token(Eof)
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
                    "a",
                ),
                value: Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "b",
                ),
                value: Some(
                    Literal(
                        Int(
                            7,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Expression(
                                Identifier(
                                    "a",
                                ),
                            ),
                            Text(
                                " and ",
                            ),
                            Expression(
                                Identifier(
                                    "b",
                                ),
                            ),
                            Text(
                                " equals ",
                            ),
                            Expression(
                                Add(
                                    Identifier(
                                        "a",
                                    ),
                                    Identifier(
                                        "b",
                                    ),
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "3 and 7 equals 10",
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