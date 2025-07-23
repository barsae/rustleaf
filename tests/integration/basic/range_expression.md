# Program 🔴
```rustleaf
// #[fail_quietly]
var x = 1..2;
```

# Output
```
None
```

# Result
```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(2)))",
)
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(DotDot),
        Token(Int, "2"),
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
                    "x",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        Literal(
                            Int(
                                2,
                            ),
                        ),
                    ),
                ),
            },
        ],
    ),
)
```

# Eval
```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(2)))",
)
```