# Program
Status: 🟢
Assertions: 3

```rustleaf
assert(-42 == -42);
assert(not true == false);
assert(not false == true);
// Bitwise NOT (~) operator has been removed from RustLeaf
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(Minus),
        3: Token(Int, "42"),
        4: Token(EqualEqual),
        5: Token(Minus),
        6: Token(Int, "42"),
        7: Token(RightParen),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Not),
        12: Token(True),
        13: Token(EqualEqual),
        14: Token(False),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Not),
        20: Token(False),
        21: Token(EqualEqual),
        22: Token(True),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Eof)
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
                        Eq(
                            Neg(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        42,
                                    ),
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
                            Not(
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ),
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
                        "assert",
                    ),
                    [
                        Eq(
                            Not(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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