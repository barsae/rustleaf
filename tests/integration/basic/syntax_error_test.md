# Program

```rustleaf
var x = ;
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
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
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
Err(
    "Unexpected token: Semicolon",
)
```

# Eval

```rust
Skipped due to parse error
```

# Output

```
Skipped due to parse error
```

# Result

```rust
Skipped due to parse error
```
