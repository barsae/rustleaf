# Program 🔴
```rustleaf
// #[fail_quietly]
"${a} and ${b} equals ${a + b}";
```

# Output
```
None
```

# Result
```rust
Err(
    "Expression not yet implemented: InterpolatedString([Expression(Identifier(\"a\")), Text(\" and \"), Expression(Identifier(\"b\")), Text(\" equals \"), Expression(Add(Identifier(\"a\"), Identifier(\"b\")))])",
)
```

# Lex
```rust
Ok(
    [
        Token(InterpolationStart),
        Token(Ident, "a"),
        Token(InterpolationEnd),
        Token(StringPart, " and "),
        Token(InterpolationStart),
        Token(Ident, "b"),
        Token(InterpolationEnd),
        Token(StringPart, " equals "),
        Token(InterpolationStart),
        Token(Ident, "a"),
        Token(Plus),
        Token(Ident, "b"),
        Token(InterpolationEnd),
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
                InterpolatedString(
                    [
                        Expression(
                            Identifier(
                                "a",
                            ),
                        ),
                        Text(
                            " and ",
                        ),
                        Expression(
                            Identifier(
                                "b",
                            ),
                        ),
                        Text(
                            " equals ",
                        ),
                        Expression(
                            Add(
                                Identifier(
                                    "a",
                                ),
                                Identifier(
                                    "b",
                                ),
                            ),
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
Err(
    "Expression not yet implemented: InterpolatedString([Expression(Identifier(\"a\")), Text(\" and \"), Expression(Identifier(\"b\")), Text(\" equals \"), Expression(Add(Identifier(\"a\"), Identifier(\"b\")))])",
)
```