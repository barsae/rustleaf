# Program
Status: 🟢
Assertions: 4

```rustleaf
// Test 'not' as unary operator
assert(not true == false);
assert(not false == true);

// Test with expressions
var x = 5;
assert(not (x > 10) == true);
assert(not (x < 3) == true);   // x=5, x<3 is false, not false is true
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(True),
        Token(EqualEqual),
        Token(False),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(False),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "10"),
        Token(RightParen),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(True),
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
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            5,
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
                        Eq(
                            Not(
                                Gt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            10,
                                        ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Not(
                                Lt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
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
    Block(
        [
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            LogicalNot(
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Bool(
                                    false,
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
                            LogicalNot(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            5,
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
                            LogicalNot(
                                Call(
                                    GetAttr(
                                        Variable(
                                            "x",
                                        ),
                                        "op_gt",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                10,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            LogicalNot(
                                Call(
                                    GetAttr(
                                        Variable(
                                            "x",
                                        ),
                                        "op_lt",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                3,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```