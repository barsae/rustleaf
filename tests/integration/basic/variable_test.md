# Program

```rustleaf
var x = 42;
print(x);
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
            token_type: Int,
            text: Some(
                "42",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "print",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: RightParen,
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
        ],
        None,
    ),
)
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
