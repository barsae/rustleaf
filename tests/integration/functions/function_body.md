# Program
Status: 🟢
Assertions: 2

```rustleaf
var z = 0;
fn add(x, y) {
    z += 1;
    x + y
}
assert(add(2, 3) == 5);
assert(z == 1);
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
        1: Token(Ident, "z"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Fn),
        6: Token(Ident, "add"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(Comma),
        10: Token(Ident, "y"),
        11: Token(RightParen),
        12: Token(LeftBrace),
        13: Token(Ident, "z"),
        14: Token(PlusEqual),
        15: Token(Int, "1"),
        16: Token(Semicolon),
        17: Token(Ident, "x"),
        18: Token(Plus),
        19: Token(Ident, "y"),
        20: Token(RightBrace),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "add"),
        24: Token(LeftParen),
        25: Token(Int, "2"),
        26: Token(Comma),
        27: Token(Int, "3"),
        28: Token(RightParen),
        29: Token(EqualEqual),
        30: Token(Int, "5"),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Ident, "assert"),
        34: Token(LeftParen),
        35: Token(Ident, "z"),
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
                    "z",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        Assignment {
                            target: Identifier(
                                "z",
                            ),
                            op: AddAssign,
                            value: Literal(
                                Int(
                                    1,
                                ),
                            ),
                        },
                    ],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "add",
                                ),
                                [
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
                            Literal(
                                Int(
                                    5,
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
                                "z",
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