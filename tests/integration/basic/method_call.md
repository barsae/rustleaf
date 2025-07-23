# Program

```rustleaf
obj.method(arg1, arg2);
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: GetAttr(Variable(\"obj\"), \"method\")",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "obj"),
        Token(Dot),
        Token(Ident, "method"),
        Token(LeftParen),
        Token(Ident, "arg1"),
        Token(Comma),
        Token(Ident, "arg2"),
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
            Expression(
                MethodCall(
                    Identifier(
                        "obj",
                    ),
                    "method",
                    [
                        Identifier(
                            "arg1",
                        ),
                        Identifier(
                            "arg2",
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
            Call(
                GetAttr(
                    Variable(
                        "obj",
                    ),
                    "method",
                ),
                [
                    Variable(
                        "arg1",
                    ),
                    Variable(
                        "arg2",
                    ),
                ],
            ),
        ],
        None,
    ),
)
```
