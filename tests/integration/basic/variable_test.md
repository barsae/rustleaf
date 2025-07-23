# Program

```rustleaf
var x = 42;
print(x);
```

# Output

```
Int(42)
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(RightParen),
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
                            42,
                        ),
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Identifier(
                            "x",
                        ),
                    ],
                ),
            ),
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
                            42,
                        ),
                    ),
                ),
            ),
        ],
        Some(
            Call(
                Variable(
                    "print",
                ),
                [
                    Variable(
                        "x",
                    ),
                ],
            ),
        ),
    ),
)
```
