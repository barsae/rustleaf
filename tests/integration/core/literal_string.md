# Program
Status: 🟢
Assertions: 3

```rustleaf
var greeting = "hello";
var empty = "";
assert(greeting == "hello");
assert(empty == "");
assert(greeting + " world" == "hello world");
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
        1: Token(Ident, "greeting"),
        2: Token(Equal),
        3: Token(String, "hello"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "empty"),
        7: Token(Equal),
        8: Token(String, ""),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "greeting"),
        13: Token(EqualEqual),
        14: Token(String, "hello"),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "empty"),
        20: Token(EqualEqual),
        21: Token(String, ""),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "greeting"),
        27: Token(Plus),
        28: Token(String, " world"),
        29: Token(EqualEqual),
        30: Token(String, "hello world"),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Eof)
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
                    "greeting",
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
                    "empty",
                ),
                value: Some(
                    Literal(
                        String(
                            "",
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
                                "greeting",
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
                                "empty",
                            ),
                            Literal(
                                String(
                                    "",
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
                                    "greeting",
                                ),
                                Literal(
                                    String(
                                        " world",
                                    ),
                                ),
                            ),
                            Literal(
                                String(
                                    "hello world",
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