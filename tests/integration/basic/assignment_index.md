# Program 🔴
```rustleaf
// #[fail_quietly]
arr[0] = 99;
```

# Output
None

# Result
```rust
Err(
    "eval not implemented for: SetItem(Variable(\"arr\"), Literal(Int(0)), Literal(Int(99)))",
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "arr"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(Equal),
        Token(Int, "99"),
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
                target: GetItem(
                    Identifier(
                        "arr",
                    ),
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
                op: Assign,
                value: Literal(
                    Int(
                        99,
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
            SetItem(
                Variable(
                    "arr",
                ),
                Literal(
                    Int(
                        0,
                    ),
                ),
                Literal(
                    Int(
                        99,
                    ),
                ),
            ),
        ],
        None,
    ),
)
```