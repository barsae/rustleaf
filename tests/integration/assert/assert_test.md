# Program 🟢
```rustleaf
assert(true);
assert(1 == 1);
assert(10 + 5 == 15, "Math should work");
print("All assertions passed!");
```

# Output
```
String("All assertions passed!")
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Plus),
        Token(Int, "5"),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(Comma),
        Token(String, "Math should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(String, "All assertions passed!"),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Literal(
                            Bool(
                                true,
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
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
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
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Math should work",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Literal(
                            String(
                                "All assertions passed!",
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
    Block(
        [
            Call(
                Variable(
                    "assert",
                ),
                [
                    Literal(
                        Bool(
                            true,
                        ),
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
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
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
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Math should work",
                        ),
                    ),
                ],
            ),
        ],
        Some(
            Call(
                Variable(
                    "print",
                ),
                [
                    Literal(
                        String(
                            "All assertions passed!",
                        ),
                    ),
                ],
            ),
        ),
    ),
)
```