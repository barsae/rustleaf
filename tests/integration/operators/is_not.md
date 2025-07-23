# Program 🔴
```rustleaf
// #[fail_quietly]
var a = 1;
a is not String;
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: String",
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
        Token(IsNot),
        Token(Ident, "String"),
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
                IsNot(
                    Identifier(
                        "a",
                    ),
                    Identifier(
                        "String",
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
            LogicalNot(
                Is(
                    Variable(
                        "a",
                    ),
                    Variable(
                        "String",
                    ),
                ),
            ),
        ),
    ),
)
```