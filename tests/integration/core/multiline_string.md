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
        Token(Var),
        Token(Ident, "multiline"),
        Token(Equal),
        Token(String, "This is a\nmultiline string\nwith multiple lines"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiline"),
        Token(BangEqual),
        Token(String, "single line"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "multiline"),
        Token(In),
        Token(Ident, "multiline"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "This is a"),
        Token(In),
        Token(Ident, "multiline"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
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
    Program(
        [
            Declare(
                "multiline",
                Some(
                    Literal(
                        String(
                            "This is a\nmultiline string\nwith multiple lines",
                        ),
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "multiline",
                            ),
                            "op_ne",
                        ),
                        [
                            Literal(
                                String(
                                    "single line",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "multiline",
                            ),
                            "op_contains",
                        ),
                        [
                            Literal(
                                String(
                                    "multiline",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "multiline",
                            ),
                            "op_contains",
                        ),
                        [
                            Literal(
                                String(
                                    "This is a",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
)
```