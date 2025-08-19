# Program
Status: 🟢
Assertions: 1

```rustleaf
var l = [1, 2, 3];
assert(1 in l);
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
        1: Token(Ident, "l"),
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
        13: Token(Int, "1"),
        14: Token(In),
        15: Token(Ident, "l"),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Eof)
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
                    "l",
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
                        In(
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Identifier(
                                "l",
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