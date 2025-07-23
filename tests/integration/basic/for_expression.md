# Program 🔴

```rustleaf
// #[fail_quietly]
for x in [1, 2, 3] {
    print(x);
}
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: For { pattern: Variable(\"x\"), iter: List([Literal(Int(1)), Literal(Int(2)), Literal(Int(3))]), body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Identifier(\"x\")]))], final_expr: None } }",
)
```

# Lex

```rust
Ok(
    [
        Token(For),
        Token(Ident, "x"),
        Token(In),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(LeftBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
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
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                    body: Block {
                        statements: [
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
                        final_expr: None,
                    },
                },
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: For { pattern: Variable(\"x\"), iter: List([Literal(Int(1)), Literal(Int(2)), Literal(Int(3))]), body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Identifier(\"x\")]))], final_expr: None } }",
)
```
