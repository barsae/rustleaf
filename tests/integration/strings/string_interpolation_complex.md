# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 10;
var y = 5;
var result = "Value: ${x + y * 2}";
assert(result == "Value: 20");
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
        3: Token(Int, "10"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "y"),
        7: Token(Equal),
        8: Token(Int, "5"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "result"),
        12: Token(Equal),
        13: Token(StringPart, "Value: "),
        14: Token(InterpolationStart),
        15: Token(Ident, "x"),
        16: Token(Plus),
        17: Token(Ident, "y"),
        18: Token(Star),
        19: Token(Int, "2"),
        20: Token(InterpolationEnd),
        21: Token(Semicolon),
        22: Token(Ident, "assert"),
        23: Token(LeftParen),
        24: Token(Ident, "result"),
        25: Token(EqualEqual),
        26: Token(String, "Value: 20"),
        27: Token(RightParen),
        28: Token(Semicolon),
        29: Token(Eof)
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
                            10,
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
                        Int(
                            5,
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
                            Text(
                                "Value: ",
                            ),
                            Expression(
                                Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Mul(
                                        Identifier(
                                            "y",
                                        ),
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
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
                                    "Value: 20",
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