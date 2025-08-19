# Program
Status: 🟢
Assertions: 3

```rustleaf
var multiline = "This is a
multiline string
with multiple lines";
assert(multiline != "single line");
assert("multiline" in multiline);
assert("This is a" in multiline);
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
        1: Token(Ident, "multiline"),
        2: Token(Equal),
        3: Token(String, "This is a\nmultiline string\nwith multiple lines"),
        4: Token(Semicolon),
        5: Token(Ident, "assert"),
        6: Token(LeftParen),
        7: Token(Ident, "multiline"),
        8: Token(BangEqual),
        9: Token(String, "single line"),
        10: Token(RightParen),
        11: Token(Semicolon),
        12: Token(Ident, "assert"),
        13: Token(LeftParen),
        14: Token(String, "multiline"),
        15: Token(In),
        16: Token(Ident, "multiline"),
        17: Token(RightParen),
        18: Token(Semicolon),
        19: Token(Ident, "assert"),
        20: Token(LeftParen),
        21: Token(String, "This is a"),
        22: Token(In),
        23: Token(Ident, "multiline"),
        24: Token(RightParen),
        25: Token(Semicolon),
        26: Token(Eof)
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
                    "multiline",
                ),
                value: Some(
                    Literal(
                        String(
                            "This is a\nmultiline string\nwith multiple lines",
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
                        Ne(
                            Identifier(
                                "multiline",
                            ),
                            Literal(
                                String(
                                    "single line",
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
                        In(
                            Literal(
                                String(
                                    "multiline",
                                ),
                            ),
                            Identifier(
                                "multiline",
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
                        In(
                            Literal(
                                String(
                                    "This is a",
                                ),
                            ),
                            Identifier(
                                "multiline",
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