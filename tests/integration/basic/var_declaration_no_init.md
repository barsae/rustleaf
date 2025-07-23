# Program

```rustleaf
var y;
```

# Output

```

```

# Result

```rust
Ok(
    Unit,
)
```

# Lex

```rust
Ok(
    [
        Token(Var),
        Token(Ident, "y"),
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
                    "y",
                ),
                value: None,
            },
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
            Declare(
                "y",
                None,
            ),
        ],
        None,
    ),
)
```
