# Program

```rustleaf
var {name, age: user_age} = user;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Var,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "age",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "user_age",
            ),
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "user",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
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

# Output

```

```

# Result

```rust
Err(
    "Complex patterns not yet implemented",
)
```
