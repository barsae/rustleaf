# Program

```rustleaf
print(1 + 2);
print(5 - 3);
print(4 * 3);
print(10 / 2);
```

# Output

```
Int(3)
Int(2)
Int(12)
Float(5.0)
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
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Minus),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Star),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Slash),
        Token(Int, "2"),
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
                        Add(
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
                        Sub(
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
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
                        Mul(
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
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
                        Div(
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
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
                    BinaryOp(
                        Add,
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
                    ),
                ],
            ),
            Call(
                Variable(
                    "print",
                ),
                [
                    BinaryOp(
                        Sub,
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                        Literal(
                            Int(
                                3,
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
                    BinaryOp(
                        Mul,
                        Literal(
                            Int(
                                4,
                            ),
                        ),
                        Literal(
                            Int(
                                3,
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
                    BinaryOp(
                        Div,
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                        Literal(
                            Int(
                                2,
                            ),
                        ),
                    ),
                ],
            ),
        ],
        None,
    ),
)
```
