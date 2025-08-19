# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = {};
assert(x is Dict);
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
        3: Token(LeftBrace),
        4: Token(RightBrace),
        5: Token(Semicolon),
        6: Token(Ident, "assert"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(Is),
        10: Token(Ident, "Dict"),
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
                    Dict(
                        [],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "Dict",
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