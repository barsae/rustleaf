# Program
Status: 🟢
Assertions: 1

```rustleaf
var name = "World";
var result = "Hello ${name}";
assert(result == "Hello World");
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
        1: Token(Ident, "name"),
        2: Token(Equal),
        3: Token(String, "World"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(StringPart, "Hello "),
        9: Token(InterpolationStart),
        10: Token(Ident, "name"),
        11: Token(InterpolationEnd),
        12: Token(Semicolon),
        13: Token(Ident, "assert"),
        14: Token(LeftParen),
        15: Token(Ident, "result"),
        16: Token(EqualEqual),
        17: Token(String, "Hello World"),
        18: Token(RightParen),
        19: Token(Semicolon),
        20: Token(Eof)
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
                    "name",
                ),
                value: Some(
                    Literal(
                        String(
                            "World",
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
                                "Hello ",
                            ),
                            Expression(
                                Identifier(
                                    "name",
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
                                    "Hello World",
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