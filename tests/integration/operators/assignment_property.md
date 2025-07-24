# Program
Status: 🔴
Assertions: 0

```rustleaf
// #[fail_quietly]
obj.field = 10;
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: obj",
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "obj"),
        Token(Dot),
        Token(Ident, "field"),
        Token(Equal),
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
            Assignment {
                target: GetAttr(
                    Identifier(
                        "obj",
                    ),
                    "field",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        10,
                    ),
                ),
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
            SetAttr(
                Variable(
                    "obj",
                ),
                "field",
                Literal(
                    Int(
                        10,
                    ),
                ),
            ),
        ],
        None,
    ),
)
```