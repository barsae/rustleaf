# Program
Status: 🟢
Assertions: 1

```rustleaf
var name = "World";
var result = "Hello ${name}!
This is a multiline string with interpolation.";
print(result);
assert(result == "Hello World!
This is a multiline string with interpolation.");
```

# Output
```
Hello World!
This is a multiline string with interpolation.
```

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
        12: Token(StringPart, "!\nThis is a multiline string with interpolation."),
        13: Token(Semicolon),
        14: Token(Ident, "print"),
        15: Token(LeftParen),
        16: Token(Ident, "result"),
        17: Token(RightParen),
        18: Token(Semicolon),
        19: Token(Ident, "assert"),
        20: Token(LeftParen),
        21: Token(Ident, "result"),
        22: Token(EqualEqual),
        23: Token(String, "Hello World!\nThis is a multiline string with interpolation."),
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
                            Text(
                                "!\nThis is a multiline string with interpolation.",
                            ),
                        ],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Identifier(
                            "result",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "Hello World!\nThis is a multiline string with interpolation.",
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