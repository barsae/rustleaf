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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(Not),
        3: Token(True),
        4: Token(EqualEqual),
        5: Token(False),
        6: Token(RightParen),
        7: Token(Semicolon),
        8: Token(Ident, "assert"),
        9: Token(LeftParen),
        10: Token(Not),
        11: Token(False),
        12: Token(EqualEqual),
        13: Token(True),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Var),
        17: Token(Ident, "x"),
        18: Token(Equal),
        19: Token(Int, "5"),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Not),
        24: Token(LeftParen),
        25: Token(Ident, "x"),
        26: Token(Greater),
        27: Token(Int, "10"),
        28: Token(RightParen),
        29: Token(EqualEqual),
        30: Token(True),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Ident, "assert"),
        34: Token(LeftParen),
        35: Token(Not),
        36: Token(LeftParen),
        37: Token(Ident, "x"),
        38: Token(Less),
        39: Token(Int, "3"),
        40: Token(RightParen),
        41: Token(EqualEqual),
        42: Token(True),
        43: Token(RightParen),
        44: Token(Semicolon),
        45: Token(Eof)
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
    RustValue(<unknown>),
)
```