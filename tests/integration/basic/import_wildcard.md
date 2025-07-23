# Program

```rustleaf
use std::*;
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

# Lex

```rust
Ok(
    [
        Token(Use),
        Token(Ident, "std"),
        Token(DoubleColon),
        Token(Star),
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
