# Program

```rustleaf
print(-42);
print(!true);
print(!false);
print(~5);
```

# Output

```
Int(-42)
Bool(false)
Bool(true)
```

# Result

```rust
Err(
    "No attribute 'op_bitwise_not' on value Int(5)",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Minus),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Bang),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Bang),
        Token(False),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Tilde),
        Token(Int, "5"),
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
                        "print",
                    ),
                    [
                        Neg(
                            Literal(
                                Int(
                                    42,
                                ),
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
                        Not(
                            Literal(
                                Bool(
                                    true,
                                ),
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
                        Not(
                            Literal(
                                Bool(
                                    false,
                                ),
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
                        BitNot(
                            Literal(
                                Int(
                                    5,
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
    Block(
        [
            Call(
                Variable(
                    "print",
                ),
                [
                    Call(
                        GetAttr(
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                            "op_neg",
                        ),
                        [],
                    ),
                ],
            ),
            Call(
                Variable(
                    "print",
                ),
                [
                    UnaryOp(
                        Not,
                        Literal(
                            Bool(
                                true,
                            ),
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "print",
                ),
                [
                    UnaryOp(
                        Not,
                        Literal(
                            Bool(
                                false,
                            ),
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
                    Call(
                        GetAttr(
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            "op_bitwise_not",
                        ),
                        [],
                    ),
                ],
            ),
        ),
    ),
)
```
