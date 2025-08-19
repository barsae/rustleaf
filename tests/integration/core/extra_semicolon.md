# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 5;;  // Extra semicolon
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
        3: Token(Int, "5"),
        4: Token(Semicolon),
        5: Token(Semicolon),
        6: Token(Ident, "assert"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(EqualEqual),
        10: Token(Int, "5"),
        11: Token(RightParen),
        12: Token(Semicolon),
        13: Token(Eof)
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
                            5,
                        ),
                    ),
                ),
            },
            Empty,
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