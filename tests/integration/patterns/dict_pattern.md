# Program
Status: 🟢
Assertions: 2

```rustleaf
var user = {"name": "Alice", "age": 30};
var {name, age: user_age} = user;
assert(name == "Alice");
assert(user_age == 30);
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
        1: Token(Ident, "user"),
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(String, "name"),
        5: Token(Colon),
        6: Token(String, "Alice"),
        7: Token(Comma),
        8: Token(String, "age"),
        9: Token(Colon),
        10: Token(Int, "30"),
        11: Token(RightBrace),
        12: Token(Semicolon),
        13: Token(Var),
        14: Token(LeftBrace),
        15: Token(Ident, "name"),
        16: Token(Comma),
        17: Token(Ident, "age"),
        18: Token(Colon),
        19: Token(Ident, "user_age"),
        20: Token(RightBrace),
        21: Token(Equal),
        22: Token(Ident, "user"),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "name"),
        27: Token(EqualEqual),
        28: Token(String, "Alice"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Ident, "user_age"),
        34: Token(EqualEqual),
        35: Token(Int, "30"),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Eof)
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
                    "user",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "name",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "Alice",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "age",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        30,
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Dict(
                    [
                        DictPattern {
                            key: "name",
                            alias: None,
                        },
                        DictPattern {
                            key: "age",
                            alias: Some(
                                "user_age",
                            ),
                        },
                    ],
                ),
                value: Some(
                    Identifier(
                        "user",
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
                                "name",
                            ),
                            Literal(
                                String(
                                    "Alice",
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
                                "user_age",
                            ),
                            Literal(
                                Int(
                                    30,
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