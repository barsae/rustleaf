# Program
Status: 🟢
Assertions: 1

```rustleaf
var empty = [];
assert(empty == []);
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
        1: Token(Ident, "empty"),
        2: Token(Equal),
        3: Token(LeftBracket),
        4: Token(RightBracket),
        5: Token(Semicolon),
        6: Token(Ident, "assert"),
        7: Token(LeftParen),
        8: Token(Ident, "empty"),
        9: Token(EqualEqual),
        10: Token(LeftBracket),
        11: Token(RightBracket),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Eof)
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
                    "empty",
                ),
                value: Some(
                    List(
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
                        Eq(
                            Identifier(
                                "empty",
                            ),
                            List(
                                [],
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