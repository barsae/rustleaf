# Program 🔴
```rustleaf
// #[fail_quietly]
var a = 1;
a is Int;
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: Int",
)
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "a"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Ident, "a"),
        Token(Is),
        Token(Ident, "Int"),
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
            VarDecl {
                pattern: Variable(
                    "a",
                ),
                value: Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            },
            Expression(
                Is(
                    Identifier(
                        "a",
                    ),
                    Identifier(
                        "Int",
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
    Block(
        [
            Declare(
                "a",
                Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            ),
        ],
        Some(
            Is(
                Variable(
                    "a",
                ),
                Variable(
                    "Int",
                ),
            ),
        ),
    ),
)
```