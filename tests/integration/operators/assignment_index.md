# Program
Status: 🟢
Assertions: 1

```rustleaf
var arr = [1, 2, 3];
arr[0] = 99;
assert(arr == [99, 2, 3]);
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
        Token(Ident, "arr"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "arr"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(Equal),
        Token(Int, "99"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "arr"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "99"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
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
                    "arr",
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
            Assignment {
                target: GetItem(
                    Identifier(
                        "arr",
                    ),
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
                op: Assign,
                value: Literal(
                    Int(
                        99,
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
                                "arr",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            99,
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
                "arr",
                Some(
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
            ),
            SetItem(
                Variable(
                    "arr",
                ),
                Literal(
                    Int(
                        0,
                    ),
                ),
                Literal(
                    Int(
                        99,
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
                                "arr",
                            ),
                            "op_eq",
                        ),
                        [
                            List(
                                [
                                    Literal(
                                        Int(
                                            99,
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
                        ],
                    ),
                ],
            ),
        ],
    ),
)
```