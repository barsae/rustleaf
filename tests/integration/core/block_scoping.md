# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = 1;
var inner_x;
var outer_x;
{
    var x = 2;
    inner_x = x;
}
outer_x = x;
assert(inner_x == 2);
assert(outer_x == 1);
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "1"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "inner_x"),
        7: Token(Semicolon),
        8: Token(Var),
        9: Token(Ident, "outer_x"),
        10: Token(Semicolon),
        11: Token(LeftBrace),
        12: Token(Var),
        13: Token(Ident, "x"),
        14: Token(Equal),
        15: Token(Int, "2"),
        16: Token(Semicolon),
        17: Token(Ident, "inner_x"),
        18: Token(Equal),
        19: Token(Ident, "x"),
        20: Token(Semicolon),
        21: Token(RightBrace),
        22: Token(Ident, "outer_x"),
        23: Token(Equal),
        24: Token(Ident, "x"),
        25: Token(Semicolon),
        26: Token(Ident, "assert"),
        27: Token(LeftParen),
        28: Token(Ident, "inner_x"),
        29: Token(EqualEqual),
        30: Token(Int, "2"),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Ident, "assert"),
        34: Token(LeftParen),
        35: Token(Ident, "outer_x"),
        36: Token(EqualEqual),
        37: Token(Int, "1"),
        38: Token(RightParen),
        39: Token(Semicolon),
        40: Token(Eof)
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "inner_x",
                ),
                value: None,
            },
            VarDecl {
                pattern: Variable(
                    "outer_x",
                ),
                value: None,
            },
            Expression(
                Block(
                    Block {
                        statements: [
                            VarDecl {
                                pattern: Variable(
                                    "x",
                                ),
                                value: Some(
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ),
                            },
                            Assignment {
                                target: Identifier(
                                    "inner_x",
                                ),
                                op: Assign,
                                value: Identifier(
                                    "x",
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                ),
            ),
            Assignment {
                target: Identifier(
                    "outer_x",
                ),
                op: Assign,
                value: Identifier(
                    "x",
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
                                "inner_x",
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
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "outer_x",
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