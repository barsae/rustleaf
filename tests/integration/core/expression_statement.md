# Program
Status: 🟢
Assertions: 1

```rustleaf
var result = 42;
result;
assert(result == 42);
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
        1: Token(Ident, "result"),
        2: Token(Equal),
        3: Token(Int, "42"),
        4: Token(Semicolon),
        5: Token(Ident, "result"),
        6: Token(Semicolon),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Ident, "result"),
        10: Token(EqualEqual),
        11: Token(Int, "42"),
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
                    "result",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            Expression(
                Identifier(
                    "result",
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
                                "result",
                            ),
                            Literal(
                                Int(
                                    42,
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