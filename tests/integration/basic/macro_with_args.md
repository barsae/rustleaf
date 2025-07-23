# Program

```rustleaf
#[test(arg1: "value", arg2: 42)]
var x = 100;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Hash,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "test",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "arg1",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "value",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "arg2",
            ),
        },
        Token {
            token_type: Colon,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "42",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
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
                "100",
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

# Output

```

```

# Result

```rust
Ok(
    Unit,
)
```
