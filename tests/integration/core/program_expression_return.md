# Program
Status: 🟡
Assertions: 0

```rustleaf
var x = 5;
var y = 3;
x + y
```

# Output
None

# Result
```rust
Ok(
    Int(8),
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
        5: Token(Var),
        6: Token(Ident, "y"),
        7: Token(Equal),
        8: Token(Int, "3"),
        9: Token(Semicolon),
        10: Token(Ident, "x"),
        11: Token(Plus),
        12: Token(Ident, "y"),
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
            VarDecl {
                pattern: Variable(
                    "y",
                ),
                value: Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            },
            Expression(
                Add(
                    Identifier(
                        "x",
                    ),
                    Identifier(
                        "y",
                    ),
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