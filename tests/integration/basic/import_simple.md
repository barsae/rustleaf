# Program

```rustleaf
use std::*;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Use,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "std",
            ),
        },
        Token {
            token_type: DoubleColon,
            text: None,
        },
        Token {
            token_type: Star,
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
Ok(
    Program(
        [
            Import(
                ImportSpec {
                    module: "std",
                    items: All,
                },
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Statement not yet implemented: Import(ImportSpec { module: \"std\", items: All })",
)
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: Import(ImportSpec { module: \"std\", items: All })",
)
```
