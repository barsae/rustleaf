# Program 🔴

```rustleaf
// #[fail_quietly]
1..10;
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(10)))",
)
```

# Lex

```rust
Ok(
    [
        Token(Int, "1"),
        Token(DotDot),
        Token(Int, "10"),
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
            Expression(
                RangeExclusive(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                    Literal(
                        Int(
                            10,
                        ),
                    ),
                ),
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(10)))",
)
```
