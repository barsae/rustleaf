# Program 🔴

```rustleaf
1..=10;
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: RangeInclusive(Literal(Int(1)), Literal(Int(10)))",
)
```

# Lex

```rust
Ok(
    [
        Token(Int, "1"),
        Token(DotDotEqual),
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
                RangeInclusive(
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
    "Expression not yet implemented: RangeInclusive(Literal(Int(1)), Literal(Int(10)))",
)
```
