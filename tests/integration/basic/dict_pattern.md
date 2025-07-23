# Program 🔴

```rustleaf
// #[fail_quietly]
var {name, age: user_age} = user;
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
        Token(LeftBrace),
        Token(Ident, "name"),
        Token(Comma),
        Token(Ident, "age"),
        Token(Colon),
        Token(Ident, "user_age"),
        Token(RightBrace),
        Token(Equal),
        Token(Ident, "user"),
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
                pattern: Dict(
                    [
                        DictPattern {
                            key: "name",
                            alias: None,
                        },
                        DictPattern {
                            key: "age",
                            alias: Some(
                                "user_age",
                            ),
                        },
                    ],
                ),
                value: Some(
                    Identifier(
                        "user",
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
