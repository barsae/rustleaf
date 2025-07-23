# Program
Status: 🟢

```rustleaf
#[test(arg1: "value", arg2: 42)]
var x = 100;
```

# Output
None

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
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(Ident, "arg1"),
        Token(Colon),
        Token(String, "value"),
        Token(Comma),
        Token(Ident, "arg2"),
        Token(Colon),
        Token(Int, "42"),
        Token(RightParen),
        Token(RightBracket),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "100"),
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            100,
                        ),
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
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            100,
                        ),
                    ),
                ),
            ),
        ],
        None,
    ),
)
```