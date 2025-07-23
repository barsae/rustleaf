# Program

```rustleaf
var [first, *rest] = [1, 2, 3, 4];
```

# Output

```

```

# Result

```rust
Err(
    "Complex patterns not yet implemented",
)
```

# Lex

```rust
Ok(
    [
        Token(Var),
        Token(LeftBracket),
        Token(Ident, "first"),
        Token(Comma),
        Token(Star),
        Token(Ident, "rest"),
        Token(RightBracket),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(Int, "4"),
        Token(RightBracket),
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
                pattern: ListRest(
                    [
                        Variable(
                            "first",
                        ),
                    ],
                    Some(
                        "rest",
                    ),
                ),
                value: Some(
                    List(
                        [
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
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                        ],
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
    "Complex patterns not yet implemented",
)
```
