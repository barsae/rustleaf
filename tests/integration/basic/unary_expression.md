# Program

```rustleaf
-42;
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: UnaryOp(Neg, Literal(Int(42)))",
)
```

# Lex

```rust
Ok(
    [
        Token(Minus),
        Token(Int, "42"),
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
                Neg(
                    Literal(
                        Int(
                            42,
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
Ok(
    Block(
        [
            UnaryOp(
                Neg,
                Literal(
                    Int(
                        42,
                    ),
                ),
            ),
        ],
        None,
    ),
)
```
