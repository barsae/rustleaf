# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 0;
x += 5;
assert(x == 5);
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
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Ident, "x"),
        6: Token(PlusEqual),
        7: Token(Int, "5"),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Ident, "x"),
        12: Token(EqualEqual),
        13: Token(Int, "5"),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Eof)
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
                            0,
                        ),
                    ),
                ),
            },
            Assignment {
                target: Identifier(
                    "x",
                ),
                op: AddAssign,
                value: Literal(
                    Int(
                        5,
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
                                "x",
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